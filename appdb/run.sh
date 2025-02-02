#!/bin/sh

set -ex

go run . -tmpl ../web/index.tmpl render > ../web/index.html
go run . -tmpl ../web/index.tmpl -broken render > ../web/broken.html
go run . deploy
