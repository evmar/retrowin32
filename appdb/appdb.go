package main

import (
	"errors"
	"flag"
	"fmt"
	"html/template"
	"io/fs"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"github.com/BurntSushi/toml"
)

var (
	flagTmplPath = flag.String("tmpl", "", "path to template for rendering")
	flagBroken   = flag.Bool("broken", false, "only list broken demos")
)

type Link struct {
	Desc string `toml:"desc"`
	URL  string `toml:"url"`
}

type Entry struct {
	Title    string `toml:"title"`
	Desc     string `toml:"desc"`
	Status   string `toml:"status"`
	Category string `toml:"category"`

	Dir string `toml:"dir"`
	// Path is the path in the retrowin32 tree to the exe.
	Path string `toml:"path"`

	// Extra files needed to run, found in Dir.
	Files []string `toml:"files"`

	// DLLs to use from the file system instead of the internal copy.
	External []string `toml:"external"`

	// Origin is where the exe came from.
	Origin *Link `toml:"origin"`

	// If present, command line to invoke the program.
	Cmdline string `toml:"cmdline"`

	// If true, doesn't work at all; omit from website unless -broken flag.
	Broken bool `toml:"broken"`
}

func loadEntry(path string) (*Entry, error) {
	var entry Entry
	if _, err := toml.DecodeFile(path, &entry); err != nil {
		return nil, err
	}
	if !entry.Broken {
		if entry.Desc == "" {
			return nil, fmt.Errorf("missing description")
		}
		if entry.Category == "" {
			return nil, fmt.Errorf("missing category")
		}
	}
	return &entry, nil
}

func load() (map[string][]*Entry, error) {
	entries := map[string][]*Entry{}
	err := filepath.WalkDir("./entries/", func(path string, d fs.DirEntry, err error) error {
		if err != nil {
			return err
		}
		if filepath.Ext(path) != ".toml" {
			return nil
		}
		entry, err := loadEntry(path)
		if err != nil {
			return fmt.Errorf("loading %q: %w", path, err)
		}
		if entry.Broken == *flagBroken {
			entries[entry.Category] = append(entries[entry.Category], entry)
		}
		return nil
	})
	if err != nil {
		return nil, err
	}

	// Sort for stability.
	for _, es := range entries {
		sort.Slice(es, func(i, j int) bool {
			return strings.ToLower(es[i].Title) < strings.ToLower(es[j].Title)
		})
	}

	return entries, nil
}

func render(entries map[string][]*Entry) error {
	if *flagTmplPath == "" {
		return fmt.Errorf("must specify -tmpl")
	}
	tmpl, err := template.ParseFiles(*flagTmplPath)
	if err != nil {
		return err
	}

	type catEntries struct {
		Category string
		Entries  []*Entry
	}

	var tmplData struct {
		Categories []*catEntries
	}

	for _, cat := range []string{"demoscene", "windows", "retrowin32 test"} {
		tmplData.Categories = append(tmplData.Categories, &catEntries{Category: cat, Entries: entries[cat]})
		delete(entries, cat)
	}
	for cat := range entries {
		return fmt.Errorf("must explicitly name category %q in appdb render", cat)
	}

	return tmpl.Execute(os.Stdout, &tmplData)
}

func deploy(entries map[string][]*Entry) error {
	if err := os.Chdir(".."); err != nil {
		return err
	}
	for _, es := range entries {
		for _, e := range es {
			if strings.HasPrefix(e.Path, "local/") {
				src := e.Path[len("local/"):]
				dst := filepath.Join("deploy/", e.Path)
				if err := os.MkdirAll(filepath.Dir(dst), 0777); err != nil {
					return err
				}
				if err := os.Link(src, dst); err != nil {
					if !errors.Is(err, fs.ErrExist) {
						fmt.Fprintf(os.Stderr, "%v\n", err)
					}
				}
			}
		}
	}
	return nil
}

func run() error {
	args := flag.Args()
	if len(args) != 1 {
		return fmt.Errorf("specify mode: {render,deploy}")
	}
	mode := args[0]

	entries, err := load()
	if err != nil {
		return err
	}

	switch mode {
	case "render":
		return render(entries)
	case "deploy":
		return deploy(entries)
	default:
		return fmt.Errorf("unknown mode %q", mode)
	}
}

func main() {
	flag.Parse()
	if err := run(); err != nil {
		fmt.Fprintf(os.Stderr, "error: %s\n", err)
		os.Exit(1)
	}
}
