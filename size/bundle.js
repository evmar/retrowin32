// db.json
var db_default = [
  {
    commit: "06251e2a5fb0f7befcb3a4f2028bf3792f935b6f",
    date: 1726288763,
    desc: "add InternetOpenA stub",
    data: {
      size: 1651562
    }
  },
  {
    commit: "9e46ce6651e43a604180befa9522bdbe89682e9e",
    date: 1726288740,
    desc: "support null hdcSrc when using BLACKNESS rop",
    data: {
      size: 1647628
    }
  },
  {
    commit: "0f94ffcb434b04b2bde616d7ab7e535d7cd7a47c",
    date: 1726287954,
    desc: "add RegSetValueExA stub",
    data: {
      size: 1647603
    }
  },
  {
    commit: "ac7a8fa78922746e25cc622f84ca1773d52ab7ae",
    date: 1726287911,
    desc: "add LoadMenuA stub",
    data: {
      size: 1646648
    }
  },
  {
    commit: "380d2e5b156dbdd769f7a2a2ee07f2c1fd63dff7",
    date: 1726287895,
    desc: "add OleInitialize stub",
    data: {
      size: 1646132
    }
  },
  {
    commit: "1aa6862203bbe4c19f513700c0f2c45ef0ea7361",
    date: 1726287664,
    desc: "add stub SetEnvironmentVariableA",
    data: {
      size: 1643553
    }
  },
  {
    commit: "b9c9243d9bd46021af8c64cb420bcfbc18e9f119",
    date: 1726261731,
    desc: "simplify vscode config",
    data: {
      size: 1642715
    }
  },
  {
    commit: "ffee128675f1936d7ac792d848065db0b5ce85a6",
    date: 1726256403,
    desc: "implement SizeofResource",
    data: {
      size: 1642715
    }
  },
  {
    commit: "ed0ee31a6a52484da0b26e1e976de5fb4e8356f5",
    date: 1726256403,
    desc: "add handles for resources",
    data: {
      size: 1645077
    }
  },
  {
    commit: "45a70cfaccd51406adf0b0ad5f24d35882278d3a",
    date: 1726256366,
    desc: "add RegQueryValueExA stub",
    data: {
      size: 1638770
    }
  },
  {
    commit: "aadfaecff4242b5cacad94a3142fdac8b0428ca7",
    date: 1726256366,
    desc: "add RegCreateKeyA stub",
    data: {
      size: 1637858
    }
  },
  {
    commit: "fdfa9003b6179d07c6cfcf380a17b2bbc61684f2",
    date: 1726256328,
    desc: "zero out entire struct in GetStartupInfoA",
    data: {
      size: 1640231
    }
  },
  {
    commit: "f3b6ace69ee575358aa5fa530cef7c6383b2ab41",
    date: 1726255883,
    desc: "use HashMap::entry",
    data: {
      size: 1640028
    }
  },
  {
    commit: "8f0477f380f7d3149b1ad6307b21959bc9df4c19",
    date: 1725484518,
    desc: "unicorn cleanups: import, pin, breakpoints",
    data: {
      size: 1640057
    }
  },
  {
    commit: "a87b9b48dcfbc3e4fa559bda27097b41d91e1deb",
    date: 1724887984,
    desc: "stash path to executable, use Path type",
    data: {
      size: 1640057
    }
  },
  {
    commit: "3ea99dcc23776c07811ae08eee470c69b26d3b96",
    date: 1724871331,
    desc: "x86 cleanups: reduce name clutter, wrapping add, use pin",
    data: {
      size: 1638015
    }
  },
  {
    commit: "71b5f5c309f74a9a9f04de4fc55c228b8290849e",
    date: 1724567337,
    desc: "simplify code64 selector",
    data: {
      size: 1638059
    }
  },
  {
    commit: "94201e6cc5ba92372d87d84356a71d708b17894a",
    date: 1724567318,
    desc: "build fix",
    data: {
      size: 1638059
    }
  },
  {
    commit: "43e5f450f605609f940183d2972750179a9d11bc",
    date: 1724534275,
    desc: "move ldt setup into x86-64 module",
    data: {
      size: 1638059
    }
  },
  {
    commit: "2446c62eb3c011330052edbff0328582994dc534",
    date: 1724351606,
    desc: "pass mem to call_async",
    data: {
      size: 1638060
    }
  },
  {
    commit: "46e58f9b88c111b09a3bcd6442cfeba4273c5876",
    date: 1723514946,
    desc: "fix regression in async, bad merge",
    data: {
      size: 1636409
    }
  },
  {
    commit: "a2abb46330b66ee4a1b2fada2e4c8d942dbff82f",
    date: 1723262240,
    desc: "remove obsolete comment",
    data: {
      size: 1633485
    }
  },
  {
    commit: "93ae6a7de18761857a60f569532bd40ca7b9114c",
    date: 1723250922,
    desc: "avoid slice of nul ptr",
    data: {
      size: 1633485
    }
  },
  {
    commit: "005f90542c923d83abcd9912695cd7148b4c7abe",
    date: 1723248726,
    desc: "fix transposed link",
    data: {
      size: 1633485
    }
  },
  {
    commit: "a5cd8da8696699ba3a5299bf32e3e96991de05a4",
    date: 1723248671,
    desc: "more docs on website",
    data: {
      size: 1633485
    }
  },
  {
    commit: "75ee095b72f93ff5ececb96dbddab1d9a2baa6dc",
    date: 1723247572,
    desc: "document how the website works",
    data: {
      size: 1633485
    }
  },
  {
    commit: "e81a39d5c990455e2c30d1139658771def522177",
    date: 1723054376,
    desc: "use pop helper for clarity",
    data: {
      size: 1633485
    }
  },
  {
    commit: "fee5c86a6b321d6048b911ea40e674ef3f9d5a9b",
    date: 1723053620,
    desc: "RtlExitUserProcess (fixes Zig binary shutdown)",
    data: {
      size: 1634115
    }
  },
  {
    commit: "817c86dd2a69f1c1dbc27615cbe168cbeb17f05c",
    date: 1723053498,
    desc: "share code for ExitProcess",
    data: {
      size: 1633612
    }
  },
  {
    commit: "112953833a9dd8e187d6a4690ba57b6e5be09889",
    date: 1722795517,
    desc: "drop snapshotting feature",
    data: {
      size: 1635378
    }
  },
  {
    commit: "bcda3c6235d2ea372b1568aa2cbc2910501f2ae3",
    date: 1722664736,
    desc: "use async fn+await rather than returning fake futures",
    data: {
      size: 1681663
    }
  },
  {
    commit: "8d61db7b02e7580b779b1bbb01284adf2bc4c592",
    date: 1722664646,
    desc: "fix unicorn build",
    data: {
      size: 1679014
    }
  },
  {
    commit: "709719b29f91256e87725f4ed0677be90e5f7e78",
    date: 1722663548,
    desc: "update ai to new cmd name",
    data: {
      size: 1679014
    }
  },
  {
    commit: "abbade7226f957d053f9bc89ff2fa696421c4fad",
    date: 1722663489,
    desc: "more ucrtbase, enough to restore gdi.exe",
    data: {
      size: 1679014
    }
  },
  {
    commit: "6225927db2a895f945d414ed081de8e442878c3a",
    date: 1722645506,
    desc: "fix rosetta build",
    data: {
      size: 1674261
    }
  },
  {
    commit: "f07723ae7378f00d28e45333d07c56f4f5e8c79d",
    date: 1722644632,
    desc: "drop hashmap for maintaining ordinals",
    data: {
      size: 1674261
    }
  },
  {
    commit: "447e46c05024f4857f13678be9e7c8dd87db8bc6",
    date: 1722643600,
    desc: "assign ordinals to all builtins",
    data: {
      size: 1675699
    }
  },
  {
    commit: "bb1412c00d305f3d3f015fc3d72aa93134594eb6",
    date: 1722627477,
    desc: "remove direct mention of shims in vtables",
    data: {
      size: 1675409
    }
  },
  {
    commit: "2d41897783e62c00f88ffc55b231b175af46f1e5",
    date: 1722627348,
    desc: "IDirectDrawSurface2::GetPixelFormat forwarding to wrong func",
    data: {
      size: 1675409
    }
  },
  {
    commit: "356f0c556bde944ee9dae3165dd822b927989f77",
    date: 1722618951,
    desc: "generate vtable symbols in builtins.rs",
    data: {
      size: 1675409
    }
  },
  {
    commit: "c9c5aef6b474dc12c6aef0d4ae37bdea1077ea0e",
    date: 1722617538,
    desc: "more orderly handling of vtable symbol names",
    data: {
      size: 1637086
    }
  },
  {
    commit: "607d25ceea99d63918ace7cc14d8c73417fa85e9",
    date: 1722611246,
    desc: "vtable symbol forwarding in dll generation",
    data: {
      size: 1637086
    }
  },
  {
    commit: "7fd7579853c7b5578a75c9b50866a760bd8087b7",
    date: 1722611103,
    desc: "parse vtable macro and generate dll vtables from it",
    data: {
      size: 1637086
    }
  },
  {
    commit: "610acb5dda66948c97e2ed4e438a18b06bc4b478",
    date: 1722611103,
    desc: "add colons to vtable macro to ease parsing",
    data: {
      size: 1636062
    }
  },
  {
    commit: "f28ed6818e8e5ccb224cbf7938774aef1f527657",
    date: 1722610927,
    desc: "write vtables as data in dlls",
    data: {
      size: 1636062
    }
  },
  {
    commit: "6907be7702e30ac141a2ea80b53658c433841442",
    date: 1722610927,
    desc: "generate entries from vtable in dlls",
    data: {
      size: 1635038
    }
  },
  {
    commit: "431ff9ab2315197566b4c765df52bf296d5795d2",
    date: 1722610927,
    desc: "rearrange fn parsing",
    data: {
      size: 1635038
    }
  },
  {
    commit: "d33c96804e528229aaa22144877e898388b6c29d",
    date: 1722610927,
    desc: "add field for dllexports vtable",
    data: {
      size: 1635038
    }
  },
  {
    commit: "ebe36fc3ebe2866e87007d4bf0a55e37f9a2d5fa",
    date: 1722610927,
    desc: "reduce use of anyhow",
    data: {
      size: 1635038
    }
  },
  {
    commit: "f87aaa38680edfeab7137a6a06ba0504221b0808",
    date: 1722525835,
    desc: "add note about biClrUsed",
    data: {
      size: 1635038
    }
  },
  {
    commit: "c89c95ed1261277347fc651c70345981196f8734",
    date: 1722525665,
    desc: "refactor bmp parsing into separate header / pixel loading",
    data: {
      size: 1635038
    }
  },
  {
    commit: "16863cf15b35a9a03bde8aec01b6d0fe999006c0",
    date: 1722524447,
    desc: "drop Mem use in bitmap parsing",
    data: {
      size: 1635740
    }
  },
  {
    commit: "ecf242ac13c41efa0628a0c610183a0f3379dfe6",
    date: 1722523484,
    desc: "reduce use of Mem",
    data: {
      size: 1636245
    }
  },
  {
    commit: "88146724c16c29dc412b7cd39393b39d98153fbc",
    date: 1722523123,
    desc: "mem.put_pod for symmetry with .get_pod",
    data: {
      size: 1636195
    }
  },
  {
    commit: "2097ea989814b803dac389813f5ac1953450b83c",
    date: 1722522814,
    desc: "change mem.slice() to return slice, not mem",
    data: {
      size: 1636782
    }
  },
  {
    commit: "dabb7144e51002171f323d40f65557b6c7fe75b5",
    date: 1722521957,
    desc: "remove unused",
    data: {
      size: 1637025
    }
  },
  {
    commit: "b02249c92828bbe56cee8f25fb22121931a0c7f5",
    date: 1722368633,
    desc: "less as_slice_todo",
    data: {
      size: 1637025
    }
  },
  {
    commit: "fc01229540d1dd11115c6709bf3dcaaf4ea8a75a",
    date: 1722368399,
    desc: "reduce use of .as_slice_todo",
    data: {
      size: 1637019
    }
  },
  {
    commit: "eac508e506f051f1326c42a096c71e1f2dfb4d5e",
    date: 1722368280,
    desc: "reduce use of Mem as slices",
    data: {
      size: 1643755
    }
  },
  {
    commit: "25876eefa60ccdb2ec15c74682ef0ac986062704",
    date: 1722301744,
    desc: "reduce use of unsafe in bitmaps",
    data: {
      size: 1644320
    }
  },
  {
    commit: "0c2071be6a243b931101413ff5f1dd44832e47cf",
    date: 1722299797,
    desc: "simplify bitmap code a bit",
    data: {
      size: 1641175
    }
  },
  {
    commit: "f5f7fa542562f46d55c45d0111f04c844d2115a9",
    date: 1722294025,
    desc: "use iter_pod for traversing initterm arrays",
    data: {
      size: 1641218
    }
  },
  {
    commit: "049c7245ee9c5ec84e764df5c09e0cfe31f91806",
    date: 1722285970,
    desc: "more ucrtbase stubs",
    data: {
      size: 1641712
    }
  },
  {
    commit: "cfe05c12b558f2239c6cc4d8a8c45e587421041c",
    date: 1722284170,
    desc: "drop lPrivate from MSG",
    data: {
      size: 1640891
    }
  },
  {
    commit: "71ba495b4bea711fb36848cbf4cbe0309c3f0623",
    date: 1722283872,
    desc: "win32: Many more implementations",
    data: {
      size: 1641160
    }
  },
  {
    commit: "6c94bd6af8810f10e860374e9d120a22dc12f0fc",
    date: 1722283724,
    desc: "win32: Fix WIN32_FIND_DATAA struct size",
    data: {
      size: 1619305
    }
  },
  {
    commit: "3ce7dbcb2b96e6e89555b8e7360de1606326432f",
    date: 1722283724,
    desc: "win32: Add WINDIR env var",
    data: {
      size: 1619509
    }
  },
  {
    commit: "f3ee19888761852479e159d122c60232515c4b96",
    date: 1722283724,
    desc: "win32: Bump heap size to 24MB",
    data: {
      size: 1619458
    }
  },
  {
    commit: "6f73d71584529a0bdac604fb80349201d053e814",
    date: 1722283724,
    desc: "cli: Log ebp in trace points",
    data: {
      size: 1619458
    }
  },
  {
    commit: "5154a77bcfb7be2aec5f6ffc469f0b2c99b2f3c9",
    date: 1722283724,
    desc: "cli: Respect ExitProcess code",
    data: {
      size: 1619458
    }
  },
  {
    commit: "49dc6677aba2cc0a4f17318cba065b7425382677",
    date: 1722283724,
    desc: "cli: Add --debug flag and reduce warning logs",
    data: {
      size: 1619458
    }
  },
  {
    commit: "3e4617b8aca925bf740e0a9d9d805b4d452e6d56",
    date: 1722283724,
    desc: "x86: Support or_r16_rm16, dec_rm16, dec_r16",
    data: {
      size: 1619458
    }
  },
  {
    commit: "f637b45780b371ed900e24e9723f000e6f37dd39",
    date: 1722283406,
    desc: "Call set_last_error in GetModuleHandleA",
    data: {
      size: 1619436
    }
  },
  {
    commit: "574662ace051ca26546d507c69d59d21f4643e3d",
    date: 1722283343,
    desc: "minor cleanups to ds/es initialization",
    data: {
      size: 1618954
    }
  },
  {
    commit: "02fa9202d6040a6a3a0894a8daa7051a0b39e278",
    date: 1722282940,
    desc: "Add LDT entries for %ds and %es",
    data: {
      size: 1618954
    }
  },
  {
    commit: "2c3dd3e329a24680b7565a1f1caaced62df6b108",
    date: 1722282929,
    desc: "Avoid debug mode panic due to misaligned address",
    data: {
      size: 1618954
    }
  },
  {
    commit: "b3ec90b49d11aaeb56e9e887e2e2f2c2c0fce8ad",
    date: 1722282524,
    desc: "fix unused warnings",
    data: {
      size: 1619289
    }
  },
  {
    commit: "5109ad688cc9a720bd8f662eeb5461cb88ec1a7f",
    date: 1722282456,
    desc: "regenerate builtins",
    data: {
      size: 1619289
    }
  },
  {
    commit: "009b4c9efcb0ba2ec373e2dc1b0dc77f374c9f7c",
    date: 1722275180,
    desc: "use slices in BitmapInfo",
    data: {
      size: 1616729
    }
  },
  {
    commit: "cd9c9fef265f7b643e141436e5b826b3c098719b",
    date: 1722274295,
    desc: "code cleanup and misc fixes",
    data: {
      size: 1616088
    }
  },
  {
    commit: "6c875d8cb275465cb47df3989092d4abce33babd",
    date: 1722274295,
    desc: "improve GDI and implement some other APIs",
    data: {
      size: 1617226
    }
  },
  {
    commit: "37ef6d9889f289aeb699ff3deb987e088d38b21b",
    date: 1722017542,
    desc: "rosetta debug note",
    data: {
      size: 1582400
    }
  },
  {
    commit: "236ed8d24bddf9c833bc7ccb1d3417377add0094",
    date: 1721938664,
    desc: "fix web build",
    data: {
      size: 1582400
    }
  },
  {
    commit: "5aeb7ab8ca02be891b0d2e8d74024590126f9a60",
    date: 1721938661,
    desc: "Use typed-path for path handling, simplify host interface",
    broken: true
  },
  {
    commit: "c2ae922e2f7a184138e26372d87da0b8b7b81cf1",
    date: 1721938628,
    desc: "Implement FileTimeToSystemTime, FormatMessageA (stub)",
    data: {
      size: 1567980
    }
  },
  {
    commit: "b72cca9025158ae2eabf6ba0f4a1c20b274c73d3",
    date: 1721937696,
    desc: "don't trace SetLastError when called internally",
    data: {
      size: 1563820
    }
  },
  {
    commit: "fdb0bf8c382f522cfb6d7a670f260da0e31f816a",
    date: 1721852539,
    desc: "fmt",
    data: {
      size: 1565605
    }
  },
  {
    commit: "692764792fe7a76d218cab751ec16cc85fde9c7f",
    date: 1721767388,
    desc: "Some CRT implementations",
    data: {
      size: 1565605
    }
  },
  {
    commit: "5fbaa210a1b3976d97fe5da321d307167a0a380d",
    date: 1721764824,
    desc: "document vscode config",
    data: {
      size: 1559705
    }
  },
  {
    commit: "ed41b888764844ea27227b69d8f39ead7925f352",
    date: 1721750996,
    desc: "change WM_MOUSEMOVE -> MOUSEMOVE",
    data: {
      size: 1559705
    }
  },
  {
    commit: "30d1bb3ea9c955b82a724f36490dbe0914af5355",
    date: 1721750996,
    desc: "fix some issues related to window messages",
    data: {
      size: 1559705
    }
  },
  {
    commit: "2f6decea26af541ca3994904d9c73df103675979",
    date: 1721700424,
    desc: "Trace return values too",
    data: {
      size: 1559396
    }
  },
  {
    commit: "5429036305d3bb0ed4aeee1b6ebbb8b668599d53",
    date: 1721698886,
    desc: "Address PR comments & add unix path mapping",
    data: {
      size: 1487456
    }
  },
  {
    commit: "61e99b133f13da0b6544a41680d92c90f119ad5e",
    date: 1721698886,
    desc: "More robust ReadFile/WriteFile impls",
    broken: true
  },
  {
    commit: "757490effb4ca14255c3765126ac41880bbd2321",
    date: 1721698886,
    desc: "Implement time APIs and proper command line",
    broken: true
  },
  {
    commit: "170cff55bcd26dd4b73f15291dff275d5234888a",
    date: 1721426491,
    desc: "more docs about how win32 works",
    broken: true
  },
  {
    commit: "90e0ce16250f2a9e649e77906e57b4882b814d79",
    date: 1721424945,
    desc: "fmt",
    broken: true
  },
  {
    commit: "0faaa1c45bb8413797f349ebd0ca13af89cd3d01",
    date: 1721244249,
    desc: "Implement GetSystemDirectoryA, GetWindowsDirectoryA",
    broken: true
  },
  {
    commit: "0d1f745a4cd3c5df1f3fa6dc1e11d52cd3434c89",
    date: 1721244249,
    desc: "Implement GlobalReAlloc, GlobalFlags",
    broken: true
  },
  {
    commit: "db15bddd1c2ef38a9c0844cac1cc072273a53559",
    date: 1721244249,
    desc: "Working FindFirstFile, FindNextFile and more",
    broken: true
  },
  {
    commit: "d3942660f4b087809c7d0ec4d87a7051069910e9",
    date: 1721244249,
    desc: "Begin implementing real file APIs",
    broken: true
  },
  {
    commit: "66f721a4cc58e050cb25dc6d7b1f6f9e60d90d40",
    date: 1721244249,
    desc: "Some winapi stubs for mwcc",
    data: {
      size: 1437467
    }
  },
  {
    commit: "91126b1eed11ee6f4cdc81e9977a5087d0fff67b",
    date: 1721196069,
    desc: "implement or_rm16_imm8",
    data: {
      size: 1430181
    }
  },
  {
    commit: "85a998741c8ca171045376aa4b9d1e870633363c",
    date: 1719596987,
    desc: "migrate to syn 2.0 api",
    data: {
      size: 1430027
    }
  },
  {
    commit: "451c348bff78c05b7d1910861fb084366b664fb9",
    date: 1719539153,
    desc: "restore logging",
    data: {
      size: 1430027
    }
  },
  {
    commit: "4bae67275a6e5f462c5577d4b54494bd47eb2cdb",
    date: 1719537063,
    desc: "use LoadLibrary to find kernel32",
    data: {
      size: 1405791
    }
  },
  {
    commit: "c3732985ff2611b58ee45430b5909e5c38929c5e",
    date: 1719536113,
    desc: "bundle dlls into executable",
    data: {
      size: 1406642
    }
  },
  {
    commit: "4e1233af73d2f86f1d1ec588b16fc3772abe32dc",
    date: 1719535810,
    desc: "consume proper stack in dlls",
    data: {
      size: 1374786
    }
  },
  {
    commit: "762f41eb856792fcdea6559e7aa32c815112f781",
    date: 1719535643,
    desc: "move argument parsing into parse module",
    data: {
      size: 1374786
    }
  },
  {
    commit: "ca433e7957f8ccfdacf4b980be1850f6e323bd65",
    date: 1719534221,
    desc: "reproducible dll builds",
    data: {
      size: 1374786
    }
  },
  {
    commit: "57f35aaebbc53b2cb7b8c0af88c799ec4bea0665",
    date: 1719534221,
    desc: "split dllexport parsing",
    data: {
      size: 1374786
    }
  },
  {
    commit: "511e5c636f687a7647be01e9596163f1468cd576",
    date: 1719533593,
    desc: "generate dll files for builtins",
    data: {
      size: 1374786
    }
  },
  {
    commit: "6726500c947947f619a1dab2ba9819fd537f5bf4",
    date: 1719530067,
    desc: "move parsing around",
    data: {
      size: 1374786
    }
  },
  {
    commit: "eaaa79cb5dd264875eab7df0b6a5609c0ea69d28",
    date: 1719527017,
    desc: "derive: use argh",
    data: {
      size: 1374786
    }
  },
  {
    commit: "ae7e340f91cbb05be096812d12d1f9ae0d96e932",
    date: 1719522696,
    desc: "refactor dllexport generation",
    data: {
      size: 1374786
    }
  },
  {
    commit: "a33af5f1ed683371adca383a109da165dd3897bd",
    date: 1719512947,
    desc: "transposed psraw/psrad",
    data: {
      size: 1374786
    }
  },
  {
    commit: "856952bd43301468406223fa2cc0c2435bcad96d",
    date: 1719429530,
    desc: "drop unneeded wrapper",
    data: {
      size: 1374786
    }
  },
  {
    commit: "4c3eed9a646bc1105ad1432c6edd170473b04b3a",
    date: 1719429324,
    desc: "drop custom log crate",
    data: {
      size: 1374792
    }
  },
  {
    commit: "5e415476f5040d6ec93295bf1c77b862038b50f3",
    date: 1719355057,
    desc: "document WM_SIZE wParam",
    data: {
      size: 1394527
    }
  },
  {
    commit: "2238651f85cc3f719ff071631ff35959ccc85a21",
    date: 1719355045,
    desc: "clip a blt",
    data: {
      size: 1394527
    }
  },
  {
    commit: "f01cfd2710076d9cb7514bf9ecd41d8f9b3276f5",
    date: 1719350888,
    desc: "psrlq",
    data: {
      size: 1394507
    }
  },
  {
    commit: "59e31b92e8fc6932edafdeb8ebdde5909cc9a299",
    date: 1719350759,
    desc: "paddd",
    data: {
      size: 1393916
    }
  },
  {
    commit: "3d64c5538929f6372ce775106468c281ad7e250f",
    date: 1719350685,
    desc: "psrad",
    data: {
      size: 1393082
    }
  },
  {
    commit: "eccf96e508b68112fc5a0a909befbe0d1a68a977",
    date: 1719350422,
    desc: "pmaddwd, refactored unpacking unsigned",
    data: {
      size: 1392455
    }
  },
  {
    commit: "55ea28ba2e657443f2a4aba4971eae1ff598112e",
    date: 1719349817,
    desc: "and rm8 variant",
    data: {
      size: 1391543
    }
  },
  {
    commit: "7767f09484726ffce99350a73f1dfc526373eed4",
    date: 1719349753,
    desc: "pcmpeqb",
    data: {
      size: 1391532
    }
  },
  {
    commit: "49a020726f090e675861d9e237cd9d1c89421aeb",
    date: 1719349077,
    desc: "propagate last error",
    data: {
      size: 1390292
    }
  },
  {
    commit: "1c1c92be7a293cd684d9678faab7664514af7053",
    date: 1719348457,
    desc: "script to autotranslate msdn docs to rust",
    data: {
      size: 1390219
    }
  },
  {
    commit: "cdcfaae0e6523cef5e6cbeac1eca8f5bcdc77a90",
    date: 1719348351,
    desc: "stash built winapi exe",
    data: {
      size: 1390219
    }
  },
  {
    commit: "53dd2655e64978348af95f37525b5a79fc6c53a2",
    date: 1719348351,
    desc: "add a 'wasm' cargo feature",
    data: {
      size: 1390219
    }
  },
  {
    commit: "7cada4847c5614863cf93c2d12e6e33ca62c80b8",
    date: 1719347302,
    desc: "silence warning",
    data: {
      size: 1390219
    }
  },
  {
    commit: "14cb84142de87110a5957510d8f1f165fdbc8d00",
    date: 1719341148,
    desc: "deploy doc",
    data: {
      size: 1390219
    }
  },
  {
    commit: "965a98da2c3fe5f0d0ffd87bef9d6cb4e9a50d46",
    date: 1719340640,
    desc: "redo build instructions, reduce use of make",
    data: {
      size: 1390219
    }
  },
  {
    commit: "76ffb4baa477ab410a1b5b58ae732e9b60f95408",
    date: 1719338582,
    desc: "document linux dependency",
    data: {
      size: 1390219
    }
  },
  {
    commit: "e292ca3b39f692a085e59c1606360cbb90a41f9e",
    date: 1719295956,
    desc: "LocalFree, return 0 from GetLastError",
    data: {
      size: 1390219
    }
  },
  {
    commit: "383fc9d9fec3fa6adab567861d19568c442eaa96",
    date: 1719295939,
    desc: "a bit more SetFilePointer",
    data: {
      size: 1389856
    }
  },
  {
    commit: "542216d9c19a5e0d4939f6e781f445bda729d924",
    date: 1719291883,
    desc: "schedule soonest ready cpu when all are blocked",
    data: {
      size: 1388443
    }
  },
  {
    commit: "e2c8d7e9780e3dc1a61e292ea6f033f0fcbf599b",
    date: 1719291656,
    desc: "make thread demo more timing sensitive",
    data: {
      size: 1388228
    }
  },
  {
    commit: "f3cf2f23b51729f3b8d4bf86fc1c06d22093275e",
    date: 1719291630,
    desc: "implement some headless timing api",
    data: {
      size: 1388228
    }
  },
  {
    commit: "e9257aa5ccf6bbbf66f37384995a48feed14fef8",
    date: 1719264773,
    desc: "mmx rewrite into pack/unpack ops, fix bug",
    data: {
      size: 1388228
    }
  },
  {
    commit: "05f449ce2395fb87dee6d9e57d9f377603c5db44",
    date: 1719096344,
    desc: "start dsound implementation",
    data: {
      size: 1388519
    }
  },
  {
    commit: "9f7a73215071dca7da4a09a4dd1da52e880b0d58",
    date: 1719091374,
    desc: "more fpu tests with some fpu bugfixes, no demos fixed",
    data: {
      size: 1392230
    }
  },
  {
    commit: "76ea7a4c4d1d6551210701ff4a84a5d940f71202",
    date: 1719019318,
    desc: "cmpxchg",
    data: {
      size: 1392125
    }
  },
  {
    commit: "82193cd0abc9615c1115fd6c25c747e0ef1356df",
    date: 1719018952,
    desc: "a bit more GetPixelFormat",
    data: {
      size: 1391949
    }
  },
  {
    commit: "3a8e4905088c21b539ce94f64613e1742851a901",
    date: 1718727605,
    desc: "macos assembly dumper",
    data: {
      size: 1391732
    }
  },
  {
    commit: "8e21d7a8fed472c6603a1da62e9e873f85915fa5",
    date: 1718727563,
    desc: "move some helper code to subdir",
    data: {
      size: 1391732
    }
  },
  {
    commit: "f7d29879156225c412863977b0fe0b8442baff7a",
    date: 1718599904,
    desc: "don't set CF in dec",
    data: {
      size: 1391732
    }
  },
  {
    commit: "2aacc6d83d959bb0e2922b8644f48b0457f91f2f",
    date: 1718571916,
    desc: "fix headless build",
    data: {
      size: 1391784
    }
  },
  {
    commit: "fbc33393eb1f7c2700248ff686d12f04716f730f",
    date: 1718512698,
    desc: "dll hmodule is now image base address",
    data: {
      size: 1391784
    }
  },
  {
    commit: "d7e650e0bf522b5a339610b1f27457277fe7aa7e",
    date: 1718502226,
    desc: "support debug breaking in shims",
    data: {
      size: 1380758
    }
  },
  {
    commit: "0d55c786ee2d670dbab1d7185e16637446d37ed2",
    date: 1718489832,
    desc: "move breakpointing out to Machine",
    data: {
      size: 1382414
    }
  },
  {
    commit: "51d09cdb9d651348a1f54452439cdfd9b09c2a11",
    date: 1718209265,
    desc: "add debugbreak cpu state",
    data: {
      size: 1380218
    }
  },
  {
    commit: "ea9c8b4919b9e683bed15309fb7501a6e6322fd5",
    date: 1718149537,
    desc: "more control over relocation addr",
    data: {
      size: 1380090
    }
  },
  {
    commit: "91ae32062c7a8fb3edd6e306cc1db080202172c6",
    date: 1718148272,
    desc: "silence warnings",
    data: {
      size: 1380066
    }
  },
  {
    commit: "7f893f3b70b6533de06730208be5700830aa3715",
    date: 1718148084,
    desc: "tidy up main a bit",
    data: {
      size: 1380066
    }
  },
  {
    commit: "e5ba2137407d47722f68db936f0022ac29347a59",
    date: 1718143994,
    desc: "split cli host module",
    data: {
      size: 1380066
    }
  },
  {
    commit: "24763b82f3d8439af3fa0e59abea8014798e6a83",
    date: 1718125427,
    desc: "unify zeroing of pod types",
    data: {
      size: 1380066
    }
  },
  {
    commit: "354a35792afd05b362a32d08b54f677bf3a59ba9",
    date: 1718095718,
    desc: "unify GetPixelFormat",
    data: {
      size: 1380080
    }
  },
  {
    commit: "980b45eca7d829231800702cafcbaf61df46f179",
    date: 1718065403,
    desc: "better failure on oom",
    data: {
      size: 1381602
    }
  },
  {
    commit: "46ac668e94129fa3ee6b2956937d93f579f75788",
    date: 1718064304,
    desc: "adjust HMODULE to appease bass.dll",
    data: {
      size: 1381328
    }
  },
  {
    commit: "659707d8557a246610a2e2b516cb050e8aa2c141",
    date: 1718062611,
    desc: "bass: log channel pos used by stream demo",
    data: {
      size: 1381313
    }
  },
  {
    commit: "b5312265f3dfc465ac001f88fdf9ad824283d6f9",
    date: 1718062399,
    desc: "stubs for file write on wasm",
    data: {
      size: 1381155
    }
  },
  {
    commit: "baa51e01749ba646298fdd04f1a6b524cc17fe34",
    date: 1718061311,
    desc: "stream: include data",
    data: {
      size: 1378722
    }
  },
  {
    commit: "e6f9e65636174373cb4124cadddaae75676f183a",
    date: 1718060791,
    desc: "missing interface member",
    data: {
      size: 1378722
    }
  },
  {
    commit: "317487e253fe409042bca24fa3656c392f77a94e",
    date: 1718060636,
    desc: "page for broken demos",
    data: {
      size: 1378722
    }
  },
  {
    commit: "07c40a281dbd17bc729135658561b1721a061860",
    date: 1718060272,
    desc: "stream demo",
    data: {
      size: 1378722
    }
  },
  {
    commit: "c045c55bd98ae6b411bcd68a6da3e092655a09a4",
    date: 1717949194,
    desc: "small Blt style tweaks",
    data: {
      size: 1378722
    }
  },
  {
    commit: "66b1a0ec0fb8458eb02ff2c036aaa8c27bf1de31",
    date: 1717903025,
    desc: "implement SetWindowPos -> ... WM_SIZE call stack",
    data: {
      size: 1378672
    }
  },
  {
    commit: "a2db97acc68fbf75601c793249e3702fed5a9368",
    date: 1717891144,
    desc: "shared better GetSurfaceDesc",
    data: {
      size: 1373257
    }
  },
  {
    commit: "76892c4181794966a37e984b668d37a9ae0f17cc",
    date: 1717825146,
    desc: "sets",
    data: {
      size: 1374921
    }
  },
  {
    commit: "69595a3d44d27e72a9ec40a4bf06aab733f6a1dc",
    date: 1717821016,
    desc: "more clipper stubs",
    data: {
      size: 1374817
    }
  },
  {
    commit: "07a0103e184c3884235b8b849b5c6000d1ea147c",
    date: 1717820637,
    desc: "clipper::SetHWnd stub",
    data: {
      size: 1374509
    }
  },
  {
    commit: "288845caefd8465516c0b82d38c8e4c2c4c516ff",
    date: 1717820443,
    desc: "move vtable macro to com module",
    data: {
      size: 1374417
    }
  },
  {
    commit: "ca1154dfca6d42fadf3ec78353a1df2f990880a2",
    date: 1717820140,
    desc: "lazy init vtables",
    data: {
      size: 1374417
    }
  },
  {
    commit: "d4221fab3380d8a3d17b024976a0036d567af46d",
    date: 1717792830,
    desc: "rearrange vtable init to not use machine directly",
    data: {
      size: 1373581
    }
  },
  {
    commit: "380b7c3d301b8ab412ef0577f913f658a3b56911",
    date: 1717791742,
    desc: "remove some shim registration indirection",
    data: {
      size: 1372928
    }
  },
  {
    commit: "3c542801f19b02964ffca151facd43b5ffeda996",
    date: 1717790819,
    desc: "rejigger vtable macro to produce better error",
    data: {
      size: 1372926
    }
  },
  {
    commit: "789542bb7e0b3a228213cd7d4f358e3b7411c6fa",
    date: 1717790039,
    desc: "IDirectDrawClipper stub",
    data: {
      size: 1362923
    }
  },
  {
    commit: "bfce5bb9dadff3681abe7905a3416cc1e0145402",
    date: 1717738812,
    desc: "split palette",
    data: {
      size: 1360565
    }
  },
  {
    commit: "7735dce47d03d81c11c4d3023b79de4f9d10fcb2",
    date: 1717738341,
    desc: "some GetDisplayMode",
    data: {
      size: 1360502
    }
  },
  {
    commit: "81eced027264c599d7e03c83cc2da7c784db086d",
    date: 1717736447,
    desc: "ddraw2",
    data: {
      size: 1359991
    }
  },
  {
    commit: "3d30812d9203aca11a6f1aeb389c0067acddc24a",
    date: 1717694651,
    desc: "more stubs"
  },
  {
    commit: "14c499638bd3d9bfa25cc09601c389659d9bc8dd",
    date: 1717694242,
    desc: "guid type and pretty print"
  },
  {
    commit: "63a590c226db99ece659689e4e5ab09f3eb4a29f",
    date: 1717692652,
    desc: "fcom"
  },
  {
    commit: "56655c5ca82ea70efbd78e0ce53f4d69ff1368c2",
    date: 1717692565,
    desc: "another bass stub"
  },
  {
    commit: "75e09051b53823f5bcef3cf71f9789050e2dac5a",
    date: 1717692552,
    desc: "QueryInterface"
  },
  {
    commit: "ad8da824612ebd0de37ada07a63dacf0aa8dc977",
    date: 1717692306,
    desc: "test ax"
  },
  {
    commit: "787a7fee7fbb5b5161f110278f2d1f8523b0d968",
    date: 1717638226,
    desc: "fsubrp"
  },
  {
    commit: "8cd96665786bae5d2001b186ecfc1571f367a3fd",
    date: 1717637984,
    desc: "event timestamps"
  },
  {
    commit: "73ca6660111ae1a6b83fa6a9f0f9b184f5278ad3",
    date: 1717637731,
    desc: "ValidateRect for whole window"
  },
  {
    commit: "785299d4a5c06985544ce20cb0e13fdfc64be429",
    date: 1717637717,
    desc: "32bpp bitmaps"
  },
  {
    commit: "08c310f745d2efce28801bdba8a1625e7455b1cb",
    date: 1717636864,
    desc: "window sizing detail"
  },
  {
    commit: "1228fdaf986bbf8aa16d57407e28ffbeb1435be8",
    date: 1717474520,
    desc: "more StretchDIBIts"
  },
  {
    commit: "0627c626fe241e43a05b37db756c959d23cbebd7",
    date: 1717473537,
    desc: "SendMessageA"
  },
  {
    commit: "1979f781c97284ab9e3051b8b83dd72735218660",
    date: 1717473030,
    desc: "comment"
  },
  {
    commit: "f466b6d5b09350b78262673701f35851943d493c",
    date: 1717472957,
    desc: "GetSystemMenu, AppendMenuA stubs"
  },
  {
    commit: "a966a5ee01de80b38110ac5c441933cde4629e70",
    date: 1717472733,
    desc: "split user32/menu"
  },
  {
    commit: "47e140895422bf29f6edfea287a0da56d14738da",
    date: 1717472583,
    desc: "CP_OEMCP"
  },
  {
    commit: "c465b67fed97d178f2071485c24ef1280d726ce6",
    date: 1717472439,
    desc: "VirtualQuery"
  },
  {
    commit: "1ac2c5f746254a52a35268a5d94f7ca9effde707",
    date: 1717464952,
    desc: "broken flag"
  },
  {
    commit: "134bf7df5f54ff5946f902c2fe3bd5a2cad11386",
    date: 1717448531,
    desc: "go mod tidy"
  },
  {
    commit: "6e8b14d007352ab937b4cdae9a15258f69170aa3",
    date: 1717448510,
    desc: "some broken opengl 4k demos"
  },
  {
    commit: "1a7de462538bcd8c74eca07974bb35d9181e9708",
    date: 1717446293,
    desc: "read_to_end"
  },
  {
    commit: "de09140730ee586161927ffab08de800d89c4de8",
    date: 1717443710,
    desc: "show window class name in trace"
  },
  {
    commit: "8e98a0873fb8f56b900355d6db7d00d5ec086c58",
    date: 1717443285,
    desc: "ignore missing wndproc"
  },
  {
    commit: "25936edf2ed84cd39a941a52beb74cf4e80d4c05",
    date: 1717441864,
    desc: "wrapping_sub to work around a glu32.dll bug(?)"
  },
  {
    commit: "98b5b702f06f7a0f723c37d37b730a4076875ca3",
    date: 1717441848,
    desc: "fix skipping absent dllmains"
  },
  {
    commit: "5a621866ae9efa77110aae40086bf8358372efab",
    date: 1717366854,
    desc: "appdb docs"
  },
  {
    commit: "8f2eae0329227f19c45bb0b76087ffd80ac030c1",
    date: 1717362622,
    desc: "ensure correct dir"
  },
  {
    commit: "88edec59897ef1a63359da489a1416a42cef59a8",
    date: 1717358209,
    desc: "move user32/misc to module"
  },
  {
    commit: "0a3ad8008fb441f60d4136885f8d4fa8577fe249",
    date: 1717357179,
    desc: "allow VirtualAlloc within existing mapping"
  },
  {
    commit: "ef7282e740f3e421ccfb15afab35772edac6b402",
    date: 1717356930,
    desc: "setg"
  },
  {
    commit: "a515623e98021f9ae7d0c4f994a3c2cca9b962c3",
    date: 1717355575,
    desc: "don't trace contents of output buffers"
  },
  {
    commit: "00ae795d0abdf61708f1265ff34e0e799465ce04",
    date: 1717350763,
    desc: "better error on reading stdin"
  },
  {
    commit: "c7dba3ee9c35aba6142a97a0e84d050ba49dbd56",
    date: 1717310738,
    desc: "json program list no longer used"
  },
  {
    commit: "d6223339ab6ebef983192f01c1c6c2b869b0daaf",
    date: 1717310459,
    desc: "two more demos"
  },
  {
    commit: "26220f1506de0a4962648cf5e67e6bdceaaa743c",
    date: 1717309886,
    desc: "add an application db of known binaries, regen homepage from it"
  },
  {
    commit: "dca462383fb8d1b15bbf5593d75d90e3979362ea",
    date: 1717304495,
    desc: "build scripts",
    data: {
      size: 1332073
    }
  },
  {
    commit: "25b54d8032968c20c10636daa39ab7391ac29ee6",
    date: 1717279856,
    desc: "resurrect no-std Windows Rust, I finally figured it out",
    data: {
      size: 1332073
    }
  },
  {
    commit: "f2a6c0ffb1aa1de9757ad97793b81f104401460a",
    date: 1717220975,
    desc: "avoid overflow in instruction count",
    data: {
      size: 1332073
    }
  },
  {
    commit: "7fc4a3ddb6f3955e1ecd1f524138de2303f7981c",
    date: 1717220969,
    desc: "pass cmdline to zip test",
    data: {
      size: 1332080
    }
  },
  {
    commit: "eaa2c0a852b72f0e55685aa08ae2bbb105e049dc",
    date: 1717218442,
    desc: "properly handle 'repe cmpsb'",
    data: {
      size: 1332080
    }
  },
  {
    commit: "389774e54621d29cee2d5fd80a0372d7539e4802",
    date: 1717103397,
    desc: "add &cmdline= url param",
    data: {
      size: 1333190
    }
  },
  {
    commit: "3ac9c02c13185973417c02ba8873830c4622cf86",
    date: 1717102594,
    desc: "obey dir= param when fetching labels csv",
    data: {
      size: 1333190
    }
  },
  {
    commit: "e0975892e0249f52ff121f7bfc3cdcb11ca2a655",
    date: 1717102594,
    desc: "slightly better start/stop around breakpoints",
    data: {
      size: 1333190
    }
  },
  {
    commit: "0df13e4d7d0dcd94c54d248eef758c85b68752ef",
    date: 1717100089,
    desc: "plumb file writing through to underlying file",
    data: {
      size: 1333190
    }
  },
  {
    commit: "41df1d7676baaed714577f2e97c381a366460cbe",
    date: 1717099864,
    desc: "rename win32::host::write",
    data: {
      size: 1331443
    }
  },
  {
    commit: "a60d3de1532362abf5d6d1713f0d302754be1cc0",
    date: 1717099843,
    desc: "allow manual dprint",
    data: {
      size: 1331469
    }
  },
  {
    commit: "a71fa8a15aacd6f6277ec503a58000c008c6a9c2",
    date: 1717099455,
    desc: "wasm build fix",
    data: {
      size: 1331469
    }
  },
  {
    commit: "779fbc936518df751db199c63c9a20e9c9fb70b6",
    date: 1717097712,
    desc: "switch kernel file set to handle map"
  },
  {
    commit: "397a08f9346b948b7075957b8acb18dc9ca4824b",
    date: 1717097272,
    desc: "plumb FileAccess::WRITE"
  },
  {
    commit: "097a08993513861a8445482436019ed15251b8c0",
    date: 1717096822,
    desc: "add FileAccess param to file open API"
  },
  {
    commit: "a42030ba05e89a632da2c121b11238e5b82726a9",
    date: 1717096662,
    desc: "don't break analyzer when xwin not installed"
  },
  {
    commit: "59efb7686fb7a00ffedeae2f3f7a534498f29fad",
    date: 1717096175,
    desc: "proper type for file access bits"
  },
  {
    commit: "2ae6cce1f2b19dc00ad2db0846d13e804c2091b8",
    date: 1717096175,
    desc: "setle"
  },
  {
    commit: "64be4bbb18218bfa9f5a3d125072e209ae5b89b7",
    date: 1717096175,
    desc: "and + or for 16-bit registers"
  },
  {
    commit: "abeab9a7afba61b02971e6386571f7dfc2e9fdb8",
    date: 1717095972,
    desc: "GetLocalTime"
  },
  {
    commit: "75d5803ad29ced97af610ea2a330b15e09f0817f",
    date: 1717095972,
    desc: "split kernel32/time, add GetTimeZoneInformation"
  },
  {
    commit: "b2ba59968090e3824e69554223155b0ae96aa757",
    date: 1717094141,
    desc: "fix logic in setge"
  },
  {
    commit: "d756b8d4bdb2965fc765a230d6461ceced7db9b1",
    date: 1717094078,
    desc: "trace: disable upx logic"
  },
  {
    commit: "8831051025e02131152a71a61792dd8f6cc59466",
    date: 1717092243,
    desc: "trace: adapt to zig update"
  },
  {
    commit: "1d8e509246805dadbba904642e34b5d8ef376f98",
    date: 1717009028,
    desc: "only pull wasm-bindgen when building wasm"
  },
  {
    commit: "378a9eb89276392c4b6abdd4d8c6d70d89d8671a",
    date: 1716962025,
    desc: "more gitignore",
    data: {
      size: 1317937
    }
  },
  {
    commit: "d694dfcad848a8bd0d293db57b886a19f6970d13",
    date: 1716962025,
    desc: "new path for cargo config",
    data: {
      size: 1317937
    }
  },
  {
    commit: "6b79fa90412c904814b6423c58876c9f32bcaa3d",
    date: 1716915438,
    desc: "memset: return dst address",
    data: {
      size: 1317937
    }
  },
  {
    commit: "fdb93206f2b614cadb5dfb16e4c1ee2c1ea4b74a",
    date: 1716915419,
    desc: "mappings debug dumper",
    data: {
      size: 1317930
    }
  },
  {
    commit: "e78ef88a295e200eb72da7bba31bf4a05895c4a9",
    date: 1716913209,
    desc: "linkify rosetta",
    data: {
      size: 1317921
    }
  },
  {
    commit: "ee32bdd146291a8338187c435277f39d65f80fdf",
    date: 1716913101,
    desc: "more docs on invocation",
    data: {
      size: 1317921
    }
  },
  {
    commit: "5b49b9b78b8995df9565ba8cca75d4dd19989d83",
    date: 1716910271,
    desc: "less public ops",
    data: {
      size: 1317921
    }
  },
  {
    commit: "75389145a971b890ae047b8453cb143dec3ffa4d",
    date: 1716908752,
    desc: "cpuid(eax=0), basic cpu info",
    data: {
      size: 1317921
    }
  },
  {
    commit: "7a9965c52433076837aa2c63fdb7fe6ccbb07d7f",
    date: 1716907234,
    desc: "parse VirtualAlloc flags",
    data: {
      size: 1317886
    }
  },
  {
    commit: "106dd51f7b2fbdc8d7827988bfecbc2e1cbd0adb",
    date: 1716906655,
    desc: "fix headless",
    data: {
      size: 1320838
    }
  },
  {
    commit: "f1d90eaf352b347f7cad84d22cba10d447035010",
    date: 1716665747,
    desc: "fmt",
    data: {
      size: 1320838
    }
  },
  {
    commit: "bdad94a5a8b3a8dad23d6236557d10dfd0307065",
    date: 1716665682,
    desc: "directly compute step size from measured instruction rate",
    data: {
      size: 1320838
    }
  },
  {
    commit: "5e10b179444edcf1d9e43f86830931431ee72817",
    date: 1716665682,
    desc: "move more breakpoints logic",
    data: {
      size: 1320838
    }
  },
  {
    commit: "119fb9ddb6aba3d0ad8888e1713214b87e6e1ab8",
    date: 1716665682,
    desc: "move breakpoint management out of emulator",
    data: {
      size: 1320838
    }
  },
  {
    commit: "063627e2418c8889240c462b2d2dd4b6c1b42dd1",
    date: 1716665682,
    desc: "use MessageChannel for loop instead of RAF",
    data: {
      size: 1320838
    }
  },
  {
    commit: "2c30985491a93ca35835890570262394c6d15af3",
    date: 1716665682,
    desc: "missed rename",
    data: {
      size: 1320838
    }
  },
  {
    commit: "ff2014e8e3d870ab4b8b0c6e1a1db0c25042bfda",
    date: 1716665682,
    desc: "pick up newest js version from esbuild",
    data: {
      size: 1320838
    }
  },
  {
    commit: "e44e05a87e7d2f500237d5e637de79ba6ec1a7f5",
    date: 1716665682,
    desc: "debugger: minor cleanups",
    data: {
      size: 1320838
    }
  },
  {
    commit: "8b7cb85f4ebae4dd3bffabda5938cbf86bf6c4e0",
    date: 1716665682,
    desc: "debugger: more fruitless refactoring",
    data: {
      size: 1320838
    }
  },
  {
    commit: "307f8cef1ffb16f1bffcc0fa4736b6195a9a6d68",
    date: 1716665682,
    desc: "debugger: refactor start/stop ui",
    data: {
      size: 1320838
    }
  },
  {
    commit: "457879610f7e60fad46e5ac9183e6fa70a7e000c",
    date: 1716665682,
    desc: "debugger: simplify tab strip",
    data: {
      size: 1320838
    }
  },
  {
    commit: "a790f9e0d451d05bdc9cdd6fb96b182a29ec6ed1",
    date: 1716665682,
    desc: "stash hwnd in surfaces",
    data: {
      size: 1320838
    }
  },
  {
    commit: "20b6591094b4bcfabfe9d20cb3895f2d505bc013",
    date: 1716665682,
    desc: "std trait for file reading",
    data: {
      size: 1320780
    }
  },
  {
    commit: "c8fe6f53dc6f90e0acf939b132f4ce263b466e72",
    date: 1716665682,
    desc: "move ts interfaces alongside types",
    data: {
      size: 1311874
    }
  },
  {
    commit: "d7376fe1fcac55b461f9e131131d36726646edd8",
    date: 1716665682,
    desc: "keep window surface in sync with size",
    data: {
      size: 1311874
    }
  },
  {
    commit: "f0a1c08e10985621ddc9c15672b320eb7a689c8c",
    date: 1716665682,
    desc: "clearer name for file name param",
    data: {
      size: 1311482
    }
  },
  {
    commit: "09110e2c856180ab268e19f59037283e1048317a",
    date: 1716089633,
    desc: "Fix a hyperlink typo",
    data: {
      size: 1311584
    }
  },
  {
    commit: "207faf234d4d85aa035c1b3bc36055be058161a5",
    date: 1715628936,
    desc: "pod put is unaligned",
    data: {
      size: 1311584
    }
  },
  {
    commit: "0e1dc7a0e2aeb511b2451967938f235d82ba7b22",
    date: 1715628727,
    desc: "avoid static mut warning",
    data: {
      size: 1311649
    }
  },
  {
    commit: "7b8d9ff6c994f15e44c5a3c4d2737e4506fa38f5",
    date: 1715627770,
    desc: "ignore sdl dll for windows",
    data: {
      size: 1311649
    }
  },
  {
    commit: "3f9f62822c0e945d218b6f72409a0cffd55164d2",
    date: 1715627741,
    desc: "windows fixes",
    data: {
      size: 1311649
    }
  },
  {
    commit: "2c561a22618736793388121d792fdcd843060729",
    date: 1715626901,
    desc: "SIGUSR1 only on unix",
    data: {
      size: 1311649
    }
  },
  {
    commit: "f6765d1742a41767593e572b71fc50bf93efa8bd",
    date: 1715471452,
    desc: "run ops test only when inputs change",
    data: {
      size: 1311649
    }
  },
  {
    commit: "df1cf00fafb0bfcb14b3325e0cd10a29b8b3699e",
    date: 1715471452,
    desc: "rename variable, for consistently calling op0 'x'",
    data: {
      size: 1311649
    }
  },
  {
    commit: "162686856b99549931b5ebdb502336bdff75c3d7",
    date: 1715471452,
    desc: "generalize fdivr, fix bug, fixes mofo",
    data: {
      size: 1311649
    }
  },
  {
    commit: "03d22cad2b2181a01be9658659e80c13c6b9395a",
    date: 1715471452,
    desc: "test f2xm1, fscale",
    data: {
      size: 1311453
    }
  },
  {
    commit: "a421b84fe1aa152cb6c1974f07f53bdc3acf83d4",
    date: 1715470500,
    desc: "fewer type coercions",
    data: {
      size: 1311453
    }
  },
  {
    commit: "206522ab279ec53807585911be46188413913e69",
    date: 1715469853,
    desc: "test add negative, fix signed bug",
    data: {
      size: 1311545
    }
  },
  {
    commit: "584678d081d1c85f1b14d217c97ac41f480c5815",
    date: 1715469565,
    desc: "test negatives in loads/stores, fix fild bug",
    data: {
      size: 1311549
    }
  },
  {
    commit: "8cd2d4c1b2b401ce1fa0adee4b0a12eab96189fb",
    date: 1715469214,
    desc: "print negative numbers in fpu tests",
    data: {
      size: 1311549
    }
  },
  {
    commit: "c7a99e13d80186dfef434c5cece86154280dadcc",
    date: 1715468503,
    desc: "tests for some fpu functions",
    data: {
      size: 1310256
    }
  },
  {
    commit: "374d0a9cf9a702df7d4f301bc447511f08ab5df3",
    date: 1715465667,
    desc: "work around weird clangd error",
    data: {
      size: 1310263
    }
  },
  {
    commit: "c32d8a3cdbd9993d2c4054078884757405e76142",
    date: 1715464649,
    desc: "yaml fmt",
    data: {
      size: 1310263
    }
  },
  {
    commit: "fa75d02c47dec1e9f5fa32846c07ea5e4595905b",
    date: 1715462221,
    desc: "convert the rest to msvc asm",
    data: {
      size: 1310263
    }
  },
  {
    commit: "5a236458cdaccbbff41688e1164c150b35ae0c44",
    date: 1715461822,
    desc: "more msvc asm",
    data: {
      size: 1151612
    }
  },
  {
    commit: "989455fbb6e25b3d66303078eb746a2ec204d210",
    date: 1715461517,
    desc: "clangd for windows code, sorta",
    data: {
      size: 1151612
    }
  },
  {
    commit: "e94ac8372ec175e6e976ec841b11724274abe20e",
    date: 1715449726,
    desc: "restore headless mode",
    data: {
      size: 1151612
    }
  },
  {
    commit: "ae6b1be4609c86d711c7cf635cc1b8d8a2eb0536",
    date: 1715449599,
    desc: "try out msvc asm",
    data: {
      size: 1151612
    }
  },
  {
    commit: "4bcd4cbcc8a00db643d3f07012cf882dba5b9632",
    date: 1715449318,
    desc: "use msvc warning flags",
    data: {
      size: 1151612
    }
  },
  {
    commit: "bd7469bf7210066432651a3aac5bde43460774d5",
    date: 1715448975,
    desc: "better crash on missing implementation",
    data: {
      size: 1151612
    }
  },
  {
    commit: "3bb04fcabf190d56bde2151c3953694d2c69d9ac",
    date: 1715448765,
    desc: "restore single step after breakpoint",
    data: {
      size: 1072714
    }
  },
  {
    commit: "2bf1daf69d21bd0c6ec7b805bdb8dedc35e86a87",
    date: 1715446505,
    desc: "resource entry names",
    data: {
      size: 1072714
    }
  },
  {
    commit: "9e4f04afd5421ac2d624c13493511a0bbbfe3e17",
    date: 1715445679,
    desc: "use underlying integer types",
    data: {
      size: 1072561
    }
  },
  {
    commit: "ca69b29b2ef667caccc174bf9d7ffab8b2ad9b8f",
    date: 1715445578,
    desc: "resource notes",
    data: {
      size: 1072561
    }
  },
  {
    commit: "f431725da5e25050347e33be3b1259186f00475a",
    date: 1715444910,
    desc: "clean up test exe list",
    data: {
      size: 1072561
    }
  },
  {
    commit: "a6034eed981bb8e2bb5ae15921873c6349767d8f",
    date: 1715267046,
    desc: "less branching in loop",
    data: {
      size: 1072561
    }
  },
  {
    commit: "8d9baf8e0902490b47a54b1bda52f35c9c8af2d7",
    date: 1715265954,
    desc: "compile-time icache size",
    data: {
      size: 1072544
    }
  },
  {
    commit: "eb8c8cdf67b2b2032dbeed8726d8cb0a0a3610db",
    date: 1715265247,
    desc: "reduce runtime checks",
    data: {
      size: 1072549
    }
  },
  {
    commit: "a3928181d412ec03b5992feddff637a30d56b44a",
    date: 1714708201,
    desc: "cache pointer to op function in instruction cache",
    data: {
      size: 1072659
    }
  },
  {
    commit: "7bef21c4626b095dfe1aeaa7d3db20a6aa532982",
    date: 1714703900,
    desc: "single step through async",
    data: {
      size: 1231396
    }
  },
  {
    commit: "b6c166aa252c616eb1b33307cbe4bf48d1ff60ef",
    date: 1714703891,
    desc: "chillin demo, my white whale",
    data: {
      size: 1231389
    }
  },
  {
    commit: "949b2646d2df3bdd35d9ca155939edb1e42f6082",
    date: 1714698957,
    desc: "more notes on bass",
    data: {
      size: 1231389
    }
  },
  {
    commit: "3d3573e48db1ffb9ff9251a15dc8aabb5dbbb5e6",
    date: 1714684681,
    desc: "--chdir flag",
    data: {
      size: 1231392
    }
  },
  {
    commit: "f2c829f1b6d2f752499593af8e482acd4489a68b",
    date: 1714683530,
    desc: "drop repr(C), we no longer cast",
    data: {
      size: 1231392
    }
  },
  {
    commit: "5b3d1f3dd51e986655ca32c4f68226e8c7581a23",
    date: 1714683511,
    desc: "tidy registers",
    data: {
      size: 1231142
    }
  },
  {
    commit: "ab81082308896581c9104567dddb739fae9ccfa6",
    date: 1714683511,
    desc: "segment registers as an array",
    data: {
      size: 1231142
    }
  },
  {
    commit: "ed0fbfccee00598c24d6f3e6e07888f65315c7dc",
    date: 1714682627,
    desc: "simpler set32",
    data: {
      size: 1233468
    }
  },
  {
    commit: "bcb3745d2d8565da199bb68f7a612159110f16c9",
    date: 1714682475,
    desc: "array-based mmx accesses",
    data: {
      size: 1233175
    }
  },
  {
    commit: "e512c60ffec32331bc970aac7ac3c6a0f36c9b14",
    date: 1714682316,
    desc: "deinline oob panics; they should never occur",
    data: {
      size: 1231893
    }
  },
  {
    commit: "8970c7cdd110857689b4a8e2b86227dcbd9c76ed",
    date: 1714682019,
    desc: "switch base 32-bit registers to an array",
    data: {
      size: 1308300
    }
  },
  {
    commit: "14527e5904dbc5393ea746e4a92b064cceb791e4",
    date: 1714680268,
    desc: "generalize access to eax",
    data: {
      size: 1309926
    }
  },
  {
    commit: "2097cd216feedf66b5189d8db9f6006f76092baf",
    date: 1714679355,
    desc: "generalize access to ecx",
    data: {
      size: 1310421
    }
  },
  {
    commit: "e87be3239efa38ee5694ddb6b91d3c12ea376f5a",
    date: 1714679079,
    desc: "simpler math",
    data: {
      size: 1311716
    }
  },
  {
    commit: "666e8615dc8c22fd89c917e232e8a6c1a5037d78",
    date: 1714678985,
    desc: "generalize access to edx",
    data: {
      size: 1311711
    }
  },
  {
    commit: "5e381f873b9a827d0c9499a30263f62081347534",
    date: 1714678178,
    desc: "factor out edx:eax accesses, possibly fixes one bug",
    data: {
      size: 1311715
    }
  },
  {
    commit: "64c2a91eaf54855aef1136e0b19617be641a2bb0",
    date: 1714677332,
    desc: "generalize access to ebx",
    data: {
      size: 1311715
    }
  },
  {
    commit: "34c2ace711323113d303d33234314c96b8f656ba",
    date: 1714677225,
    desc: "generalize some register access",
    data: {
      size: 1311715
    }
  },
  {
    commit: "25e199ff39604bd0bb2d59e5fbfd6bfde1c1083f",
    date: 1714676769,
    desc: "generalize some register access",
    data: {
      size: 1310086
    }
  },
  {
    commit: "17643bcc08605f02e623430c630817c5de739fdc",
    date: 1714676620,
    desc: "generalize some register access",
    data: {
      size: 1310102
    }
  },
  {
    commit: "c9912d13bafee0f147ca975c59831a71e329a6a2",
    date: 1714676471,
    desc: "generalize some register access",
    data: {
      size: 1310102
    }
  },
  {
    commit: "6abd6091ccabce4ed54f758d63dc86fea88623e0",
    date: 1714676345,
    desc: "generalize some register access",
    data: {
      size: 1310086
    }
  },
  {
    commit: "a17080b5d39f18c69392da5b7bc4ebb87e7f3d10",
    date: 1714675687,
    desc: "import iced just within register logic",
    data: {
      size: 1314078
    }
  },
  {
    commit: "f9c1c2718e07973ab13caaa55fa88dc8535d6e55",
    date: 1714675513,
    desc: "simplify/regularize register access",
    data: {
      size: 1314078
    }
  },
  {
    commit: "9e98960d8dc9bd1efb8adaff96f54289811231b4",
    date: 1714661920,
    desc: "relocate blocking future code to cpu",
    data: {
      size: 1314895
    }
  },
  {
    commit: "0537e9bbb6e04bc0c78d10b4d77b3cb71798d100",
    date: 1714661570,
    desc: "implement Sleep using new blocking api",
    data: {
      size: 1314698
    }
  },
  {
    commit: "49731cee2e4aadb3ad190f0a6a94b0e782725f48",
    date: 1714151228,
    desc: "redesign cpu blocking for multiple threads",
    data: {
      size: 1332527
    }
  },
  {
    commit: "29e6ffc48ac16c6e0fe2fc287f869c8ea2c717a2",
    date: 1713990904,
    desc: "fix crash when cpu pointers invalidated",
    data: {
      size: 1310709
    }
  },
  {
    commit: "c7e48768daee3fdf67e6a70e20de7f12c94f5ac2",
    date: 1713940678,
    desc: "move async handling to x86 crate",
    data: {
      size: 1309779
    }
  },
  {
    commit: "4db6ccdcff2b4c872f10e5dc806e75ee10d5db48",
    date: 1713829596,
    desc: "drop last bit of msvc",
    data: {
      size: 1310949
    }
  },
  {
    commit: "55f58cabd977c7e1d7e47c818e5badc974b25db3",
    date: 1713740251,
    desc: "another sub variant",
    data: {
      size: 1310949
    }
  },
  {
    commit: "1dfb0476765c38a69c1ba6a6534486a6d5320bc7",
    date: 1713733431,
    desc: "StretchDIBits",
    data: {
      size: 1310938
    }
  },
  {
    commit: "6c536a7cb0ffb885be0cf59a25fe2232f96ab44b",
    date: 1713733058,
    desc: "waveOutWrite",
    data: {
      size: 1307966
    }
  },
  {
    commit: "dc031d8d7115b224102df70980e3c21094fbd06e",
    date: 1713733014,
    desc: "waveOutPrepareHeader",
    data: {
      size: 1306968
    }
  },
  {
    commit: "ba2dd8597d1c495e23fc4c52f5efcdef6ce43142",
    date: 1713732834,
    desc: "waveOutGetPostition",
    data: {
      size: 1305156
    }
  },
  {
    commit: "2397e85d8f2eded6792bd7b5efa34479473ff291",
    date: 1713732349,
    desc: "SetWindowText",
    data: {
      size: 1301431
    }
  },
  {
    commit: "d538f168a152adb81a60269e4267fa5c839a0c28",
    date: 1713732116,
    desc: "div16, div8",
    data: {
      size: 1301718
    }
  },
  {
    commit: "2f3a24b967fcb69c69a2c7c685c0770b5af23ac1",
    date: 1713731250,
    desc: "note on why print",
    data: {
      size: 1301387
    }
  },
  {
    commit: "a5d70c058c39adb32498176ff5065e1de55b0f47",
    date: 1713730571,
    desc: "threading: basic thread works!?",
    data: {
      size: 1301387
    }
  },
  {
    commit: "f87afc21a00c109b0b0492cabd89d89646e2ade5",
    date: 1713726036,
    desc: "threading: add test program",
    data: {
      size: 1300382
    }
  },
  {
    commit: "573a21ad59d6707aa92356a290d8dac0b91baa87",
    date: 1713725384,
    desc: "more exe notes",
    data: {
      size: 1300382
    }
  },
  {
    commit: "9fd664737ac44281c780157873fc4c6ec36bc7e9",
    date: 1713719511,
    desc: "move gdi to a crate of multiple bins",
    data: {
      size: 1300382
    }
  },
  {
    commit: "8a852774d0acb7862a623bd3cc2f643c64e68e52",
    date: 1713719511,
    desc: "louder failure if missing env var",
    data: {
      size: 1300382
    }
  },
  {
    commit: "39cf37814cb0bf6dc2ccc31ff10321b7a4dfd7e5",
    date: 1713719511,
    desc: "restore instruction counting",
    data: {
      size: 1300382
    }
  },
  {
    commit: "12ebb904752cba273f8f40f9c0f64219ea08cee9",
    date: 1713719511,
    desc: "threading: change cpu to array",
    data: {
      size: 1300356
    }
  },
  {
    commit: "ec682670900235985a69531b8b29d51a24a04330",
    date: 1713714842,
    desc: "tidying",
    data: {
      size: 1296702
    }
  },
  {
    commit: "0b18191bec0448080f16b69778c318c7536ac22c",
    date: 1713714635,
    desc: "move instruction count up to x86 struct",
    data: {
      size: 1296702
    }
  },
  {
    commit: "032d8413c88c6ecc5a5103ebdfd91add5d09475f",
    date: 1713712770,
    desc: "Get/SetPixel stubs",
    data: {
      size: 1296928
    }
  },
  {
    commit: "a6cba92af08d6fcf963d4f2c67bb5a339c5d2444",
    date: 1713710739,
    desc: "note on compatible bitmap",
    data: {
      size: 1295294
    }
  },
  {
    commit: "8cae452950c15b60fe9f847db850ea565c5144f1",
    date: 1713708681,
    desc: "RestoreDisplayMode",
    data: {
      size: 1294660
    }
  },
  {
    commit: "939ff2d9ffe3161b2c7939b78ff93ab1a3ce330f",
    date: 1713708418,
    desc: "fix trace module name",
    data: {
      size: 1294516
    }
  },
  {
    commit: "a4da1cf15984c07ea053800333b086c0adf17bfc",
    date: 1713708394,
    desc: "COM Release",
    data: {
      size: 1294500
    }
  },
  {
    commit: "3c49fae2a202e06fd90bbdb9c7875d4e7508d0f6",
    date: 1713707597,
    desc: "stray log statement",
    data: {
      size: 1294423
    }
  },
  {
    commit: "d847a24a9952209df851f40db5c608eb1dc9c044",
    date: 1713707300,
    desc: "winmm: start on sound",
    data: {
      size: 1292537
    }
  },
  {
    commit: "e0e40f60561d3ed0c01087cdc86ff2c7f2e33280",
    date: 1713681419,
    desc: "disable fullscreen, too annoying",
    data: {
      size: 1285935
    }
  },
  {
    commit: "e05d5330b836ecb6ac0dab7949f4701d2d5da41e",
    date: 1713681030,
    desc: "reduce fullscreen panic to warning",
    data: {
      size: 1285935
    }
  },
  {
    commit: "3e282e6c509444c6502ef9f690a211fb067054bb",
    date: 1713680188,
    desc: "much simpler instruction cache",
    data: {
      size: 1285826
    }
  },
  {
    commit: "dbcae655efdbbb08bd9be05823e40fbbf9a1bd9d",
    date: 1713677062,
    desc: "simplify x86_addr computation",
    data: {
      size: 1302749
    }
  },
  {
    commit: "852c7057b784c18e8dd8e399a93fed7912726770",
    date: 1713664895,
    desc: "fullscreen",
    data: {
      size: 1303087
    }
  },
  {
    commit: "bb55c2aca36e5b7c3f7b8bc058f4fad5dba692d8",
    date: 1713639096,
    desc: "update status in readme",
    data: {
      size: 1304666
    }
  },
  {
    commit: "df0e1c76fbb09aa58d16b3a71b9b96b300a82ef8",
    date: 1713638743,
    desc: "migrate zip test to rust",
    data: {
      size: 1304666
    }
  },
  {
    commit: "a30305a3edcd8c99af7eedf7550fe8d80701c605",
    date: 1713638743,
    desc: "adc",
    data: {
      size: 1304666
    }
  },
  {
    commit: "962abc9b0854781aed333cfdc5f96aa245a3fad6",
    date: 1713638743,
    desc: "even more careful shifting",
    data: {
      size: 1304482
    }
  },
  {
    commit: "dea07cfdfc9710191ad21bed51ea057fae84f0a6",
    date: 1713638743,
    desc: "tzcnt",
    data: {
      size: 1304484
    }
  },
  {
    commit: "ad8bdfe7ee2cc8ab3994486bbe1ce04711839470",
    date: 1713638743,
    desc: "drop cwd mangling in cli",
    data: {
      size: 1304350
    }
  },
  {
    commit: "d6a77d0c30e9a4c51805dac027c78203d9b540a1",
    date: 1713638743,
    desc: "more careful masking in bit shifts",
    data: {
      size: 1304350
    }
  },
  {
    commit: "e156883136d3f4bcd6c57db61c776112a97b769c",
    date: 1713638743,
    desc: "CloseHandle",
    data: {
      size: 1304342
    }
  },
  {
    commit: "41e5f50bd569170c2f5cbbf7c0d442224e893936",
    date: 1713638743,
    desc: "ntdll!NtReadFile",
    data: {
      size: 1303341
    }
  },
  {
    commit: "8ce11c01990bf97f7097e33b96655cea838acf2f",
    date: 1713638743,
    desc: "memcmp",
    data: {
      size: 1297813
    }
  },
  {
    commit: "7b5c1152529e31245604939f9b6b4e0a01c4a2df",
    date: 1713638743,
    desc: "setbe",
    data: {
      size: 1297626
    }
  },
  {
    commit: "36ae2c6f0caae95aa1dacd8e1adb92e7bd751741",
    date: 1713638743,
    desc: "more shld,shrd",
    data: {
      size: 1297529
    }
  },
  {
    commit: "a731aabb804065b063ff208ba71811a96c39350f",
    date: 1713638743,
    desc: "setae, bts",
    data: {
      size: 1297030
    }
  },
  {
    commit: "232b7edb8ade61d781214229229346b118cb44f4",
    date: 1713638743,
    desc: "GetFileInformationByHandle",
    data: {
      size: 1296796
    }
  },
  {
    commit: "4d3baf519030722371bdbe2872b781a5131f3962",
    date: 1713632639,
    desc: "more flags in CreateFile",
    data: {
      size: 1292675
    }
  },
  {
    commit: "612e2724c355ec77d68fe2216a1785c488bff8b5",
    date: 1713632602,
    desc: "deref for Str16 etc",
    data: {
      size: 1291872
    }
  },
  {
    commit: "f9c98950e0d88dc7819c9a32ae2c8e80fafc231c",
    date: 1713630568,
    desc: "stray logging statement",
    data: {
      size: 1292591
    }
  },
  {
    commit: "533f4adb2ca3e809104b57ecfb48371b5d9c0f8a",
    date: 1713630467,
    desc: "_CxxThrowException",
    data: {
      size: 1294764
    }
  },
  {
    commit: "34728c6308018525b6123ecb82bb3ba020a81f66",
    date: 1713630291,
    desc: "more not variants",
    data: {
      size: 1293827
    }
  },
  {
    commit: "4ae48148d18c3e2fedfab33a9dc611570273e62f",
    date: 1713630143,
    desc: "another stub",
    data: {
      size: 1293656
    }
  },
  {
    commit: "a06439991e581acc3477e61be2f0a66e7609daac",
    date: 1713630131,
    desc: "FormatMessageW",
    data: {
      size: 1291599
    }
  },
  {
    commit: "3c82c3228852738de14d1a815eb15d2fb0b7b956",
    date: 1713627171,
    desc: "GetFullPathNameW, memset",
    data: {
      size: 1291999
    }
  },
  {
    commit: "11672cbdb777d5dbead1e9e87181b58577ac9840",
    date: 1713597475,
    desc: "InitializeCriticalSectionEx stub",
    data: {
      size: 1288265
    }
  },
  {
    commit: "b234db0d7b405f068b4b1649251ef32d2f241be1",
    date: 1713574106,
    desc: "more std in gdi",
    data: {
      size: 1284641
    }
  },
  {
    commit: "5fc9a0bc69cafe82324767e3569e4028b79a2b61",
    date: 1713573357,
    desc: "move gdi app to use rust std",
    data: {
      size: 1284641
    }
  },
  {
    commit: "4a25bb1231e635d6a5323f295f606fc795e4433b",
    date: 1713573266,
    desc: "cargo update windows-targets to pick up i586 fix",
    data: {
      size: 1284641
    }
  },
  {
    commit: "0ec125bd6897523252b81f698e373d5d95a5060a",
    date: 1713247058,
    desc: "fix comments",
    data: {
      size: 1284641
    }
  },
  {
    commit: "b9e58397c6bb6a8339d1bebe7c24a9488352b759",
    date: 1713246907,
    desc: "more winapi for rust hello world",
    data: {
      size: 1284641
    }
  },
  {
    commit: "4a3f933639342b3cc2b06e5d249a2c2e2b5936d1",
    date: 1713240908,
    desc: "build x86 rust as i586 to avoid sse",
    data: {
      size: 1283605
    }
  },
  {
    commit: "357abfb954976631f746d7483a0c2dd60924d958",
    date: 1713240789,
    desc: 'treat int3 as "blocked" cpu state, not error',
    data: {
      size: 1283605
    }
  },
  {
    commit: "c6fe6730ccab6d3bec8354b2d86662de4cc638d0",
    date: 1713240758,
    desc: "explicitly annotate cdecl functions",
    data: {
      size: 1283552
    }
  },
  {
    commit: "cdba81befe9dd2fc94b3a4de4d788f36bcf74aa1",
    date: 1713240688,
    desc: "wrong SetThreadDescription signature",
    data: {
      size: 1283552
    }
  },
  {
    commit: "9759da8bbdb301fe6fa6133329f18829f8308fe3",
    date: 1713113650,
    desc: "demo fpu test",
    data: {
      size: 1283377
    }
  },
  {
    commit: "b9775cdede2de38ad23693dfea65a7940c22b07f",
    date: 1713054011,
    desc: "split ops binary into modules",
    data: {
      size: 1283377
    }
  },
  {
    commit: "37413e2ba12dafb82c1df86e3e1c19de12bd0439",
    date: 1713052938,
    desc: "clarify behavior/docs on 1-bit shifts",
    data: {
      size: 1283377
    }
  },
  {
    commit: "9ffad2350364e1260c80459698d2e18ced46526f",
    date: 1713052741,
    desc: "OF is undefined for shifts != 1",
    data: {
      size: 1283571
    }
  },
  {
    commit: "2c207310865e45ddc3c84c6dcdc1bedaf2d0c85d",
    date: 1713046571,
    desc: "restore headless build",
    data: {
      size: 1283571
    }
  },
  {
    commit: "49c799e5c1b8ad7708cdbc9303e89a9210899f53",
    date: 1713046426,
    desc: "move to cross-compiling ops test",
    data: {
      size: 1283571
    }
  },
  {
    commit: "c8125daf1dcd7a62af033cefaa8571b1f87dee17",
    date: 1713044239,
    desc: "reproducible ops.exe",
    data: {
      size: 1283571
    }
  },
  {
    commit: "c18d6b12d19192d875f05448efd567f3d71c4612",
    date: 1712955133,
    desc: "pass (not working)? /Brepro",
    data: {
      size: 1283571
    }
  },
  {
    commit: "e28c3bdedfa122500d62d54c9d94adeb40632247",
    date: 1712953801,
    desc: "cross-compile ops test with clang",
    data: {
      size: 1283571
    }
  },
  {
    commit: "e84c7a156a4b091714c3def5e0787f361fe9faf7",
    date: 1712951669,
    desc: "some 8-bit ops",
    data: {
      size: 1283571
    }
  },
  {
    commit: "8ac0af66369e9cb42a7e09d27261ad771387dc34",
    date: 1712877253,
    desc: "LoadLibrary etc .dll suffix is optional",
    data: {
      size: 1282774
    }
  },
  {
    commit: "0e462dc2f93d707dce2392d6a5b34ec1158b4fca",
    date: 1712877247,
    desc: "free old mem in realloc",
    data: {
      size: 1282604
    }
  },
  {
    commit: "1985be4a15e5c304f9239ab71b08c0f325210555",
    date: 1712877243,
    desc: "SetThreadDescription",
    data: {
      size: 1282588
    }
  },
  {
    commit: "10d49de98edcb657549ef71c6dc1f9f2f58cd79b",
    date: 1712864340,
    desc: "label GetProcAddr imports",
    data: {
      size: 1282174
    }
  },
  {
    commit: "20fbbcd5fdecbbaa4068ab407da1a36853aad2d6",
    date: 1712810390,
    desc: "better debug state when in shim",
    data: {
      size: 1281872
    }
  },
  {
    commit: "a37c3af325c8be798ec9c635909d2ff507e8de85",
    date: 1712809466,
    desc: "fix single stepping over shims",
    data: {
      size: 1281872
    }
  },
  {
    commit: "95b2045c36d45ae7d3a1c5697a4a89051bad698a",
    date: 1712705966,
    desc: "more mmx ops",
    data: {
      size: 1282892
    }
  },
  {
    commit: "55641f0c610d3eb1745c335736d796f2c8c833ea",
    date: 1712705195,
    desc: "ValidateRect stub",
    data: {
      size: 1278650
    }
  },
  {
    commit: "fd52337570b81cf81aada759692636794246d02d",
    date: 1712699761,
    desc: "SetTimer support in web",
    data: {
      size: 1277969
    }
  },
  {
    commit: "9c7cf4624f5df03e5626f17ee2fd71490633b140",
    date: 1712695768,
    desc: "newlines in log messages",
    data: {
      size: 1277931
    }
  },
  {
    commit: "f64804f8f519abab9c971b50898375a2f56b0ecf",
    date: 1712695498,
    desc: "lodsw typo",
    data: {
      size: 1277931
    }
  },
  {
    commit: "f3b99a3f092ecff3a2ca2e5a4249f70153f7a417",
    date: 1712695147,
    desc: "simplify single-step logic",
    data: {
      size: 1277801
    }
  },
  {
    commit: "a4180a320a13709c912958f8735073c5ff119951",
    date: 1712692812,
    desc: "show stdout/stderr in non-debugger ui",
    data: {
      size: 1278028
    }
  },
  {
    commit: "1092eecb5bdb2756df6efb7da7aa8bf33f89dc54",
    date: 1712689170,
    desc: "comments",
    data: {
      size: 1278028
    }
  },
  {
    commit: "e09dfc98f6d3c6c33a50a28a7f44e371aac49159",
    date: 1712611358,
    desc: "more imul",
    data: {
      size: 1278028
    }
  },
  {
    commit: "6769a009a074480852bd71c0c904d7fb0db1ba79",
    date: 1712611358,
    desc: "more idiv",
    data: {
      size: 1277937
    }
  },
  {
    commit: "2fb99b64569ec3d2d6dd3c7e2d2f3a50792eeaae",
    date: 1712550115,
    desc: "generic mul",
    data: {
      size: 1277269
    }
  },
  {
    commit: "6d4e4e2ce2cc8f6a8c872f156999a0672c927f6d",
    date: 1712549407,
    desc: "disable dsound",
    data: {
      size: 1276977
    }
  },
  {
    commit: "e6667d15f3a2d7704f87331c8e0d0a8039bb5fbf",
    date: 1712549317,
    desc: "remove unneeded underscores",
    data: {
      size: 1289970
    }
  },
  {
    commit: "6f65001110ab8c114ed4cfb48f5ed309f1b2e7bb",
    date: 1712548858,
    desc: "more dsound stubs",
    data: {
      size: 1290002
    }
  },
  {
    commit: "67ee460861c293c857fc8a26d82b33618d5068d7",
    date: 1712548507,
    desc: "shld",
    data: {
      size: 1289188
    }
  },
  {
    commit: "b1bfe54f6812875419e7fea4ec8b3b87232cc82a",
    date: 1712546369,
    desc: "some stubs for heaven7",
    data: {
      size: 1288972
    }
  },
  {
    commit: "bc0f25b35271f509212c286def8a8131c8da6827",
    date: 1712537051,
    desc: "split dialog code",
    data: {
      size: 1286288
    }
  },
  {
    commit: "c71a1f32da2365bd17e801908705a44eea2fef16",
    date: 1712536881,
    desc: "cpuid",
    data: {
      size: 1287656
    }
  },
  {
    commit: "b4edb14f2aa6d6472bdc93cf816d41782efc5e04",
    date: 1712535114,
    desc: "use generalized rep for all string ops",
    data: {
      size: 1287423
    }
  },
  {
    commit: "dfa35a7739bd32fa1418c9084da36087b95e54ba",
    date: 1712516837,
    desc: "DirectSoundEnumerateA stub",
    data: {
      size: 1282584
    }
  },
  {
    commit: "bc9ab7416fa56d9dda23c685ea5404a8d667c182",
    date: 1712515836,
    desc: 'wsprintf("%s")',
    data: {
      size: 1282653
    }
  },
  {
    commit: "80f5e0739495c307de9885de425ef8dd52781476",
    date: 1712515663,
    desc: "stos with direction flag, also generalized",
    data: {
      size: 1281985
    }
  },
  {
    commit: "45931d50e7d13ca629cafb3549c0913669b2bb82",
    date: 1712466737,
    desc: "restore minesweeper",
    data: {
      size: 1281249
    }
  },
  {
    commit: "f86ba2a87f231d403e7d64901438f3260ffab33e",
    date: 1712456646,
    desc: "possibly faster get_ptr logic",
    data: {
      size: 1281205
    }
  },
  {
    commit: "fa68e186af7805112c4467e349a25748c8c25655",
    date: 1712455366,
    desc: "another fpu op",
    data: {
      size: 1275562
    }
  },
  {
    commit: "09873f3e56ebd97e11a7681fd7620a35a998bb8e",
    date: 1712454722,
    desc: "trace win32 api only in debugger",
    data: {
      size: 1275300
    }
  },
  {
    commit: "1a717f6a09262aa680fa60f5ba79b33fb6f34e05",
    date: 1712423196,
    desc: "ensure window pixels always have full alpha",
    data: {
      size: 1275047
    }
  },
  {
    commit: "d231aa5ba8e317e823fee60e14181df5596a7888",
    date: 1712415479,
    desc: "don't log on fpu exception",
    data: {
      size: 1274723
    }
  },
  {
    commit: "076c3e2e8885592996af777f0c6f73e4cfe749ab",
    date: 1712415433,
    desc: "fix wasm build",
    data: {
      size: 1275346
    }
  },
  {
    commit: "2a460baee5c5578aba722b96a77c4a5bf2ae76e7",
    date: 1712414987,
    desc: "don't erase background in paint if not requested"
  },
  {
    commit: "e408a830494138ea68edf7a833ebbe40c608382a",
    date: 1712386229,
    desc: "first pass at timers"
  },
  {
    commit: "f1da50064a5a9be9ccb45beb5374ffe5ea16ea5b",
    date: 1712377334,
    desc: "working on anatyda",
    data: {
      size: 1272918
    }
  },
  {
    commit: "86e87cf7349e8359cc41be532bc11cd872194eef",
    date: 1712377298,
    desc: "fix web lto build",
    data: {
      size: 1272918
    }
  },
  {
    commit: "1c5106056874d5a02bd705876f921597ad295db1",
    date: 1712377283,
    desc: "InvalidateRgn",
    data: {
      size: 1434600
    }
  },
  {
    commit: "ff28f91001739e6228d6859421ce160b785cebd0",
    date: 1712377215,
    desc: "bfix",
    data: {
      size: 1434600
    }
  },
  {
    commit: "f0e8ccd56fbfbc6909a6727aec6f273b148509ed",
    date: 1712363554,
    desc: "restore unicorn",
    data: {
      size: 1434600
    }
  },
  {
    commit: "44d34ad7958de03b876acc66e97eb43f4f91ec4f",
    date: 1712360721,
    desc: "fix rosetta warnings",
    data: {
      size: 1434600
    }
  },
  {
    commit: "5706ad100fdecd0d878753904ec47856fb67dc61",
    date: 1712360284,
    desc: "log and avoid fpu stack under/overflow",
    data: {
      size: 1434600
    }
  },
  {
    commit: "a41f4c33b2dd9be0ed9ae91df74aebd043f1e04e",
    date: 1712273374,
    desc: "refactor",
    data: {
      size: 1434600
    }
  },
  {
    commit: "c2548e880afb4092368e7f5a0a9b4f62e88e3fc1",
    date: 1712272505,
    desc: "fpu pop",
    data: {
      size: 1434600
    }
  },
  {
    commit: "55a4442e8e3dc640b40169a6d20486b75df266c3",
    date: 1712272002,
    desc: "fpu push",
    data: {
      size: 1434600
    }
  },
  {
    commit: "387652d1871e8e5e17c4d997d0eee77d5e0a7b88",
    date: 1712270934,
    desc: "separate lto profile",
    data: {
      size: 1434600
    }
  },
  {
    commit: "b59ab05838532028cd7802ccccb69f7e4bff7171",
    date: 1712270915,
    desc: "split fpu registers",
    data: {
      size: 1270991
    }
  },
  {
    commit: "6524a2b0c68e37b0adaaa2427c2bcc457f60a67c",
    date: 1711933848,
    desc: "caller passes in disassembly output limit"
  },
  {
    commit: "9a71969738a043215c659d0e70ecf5e880f1dc34",
    date: 1711339184,
    desc: "a bit more bitmaps, clipping BltBlt"
  },
  {
    commit: "05cf0e1e24e8c4e0722b3f8a9665cd82637b41b6",
    date: 1711337376,
    desc: "bitmap with x86 pixeldata"
  },
  {
    commit: "a5acbf36fc631c4eac452e3be89b3b15e2b0473f",
    date: 1711332817,
    desc: "allow for no active window"
  },
  {
    commit: "cc7bb23a37e527dd023f9cd1508b610667d362d5",
    date: 1711319157,
    desc: "FreeLibrary"
  },
  {
    commit: "ee426f5c63a7b5447aaaf00da8549f0b1bf41c02",
    date: 1711318906,
    desc: "ignore any unspecified GlobalAlloc/LocalAlloc flags"
  },
  {
    commit: "e442fa33c0437642be41896182a9793de4d20bb6",
    date: 1711315892,
    desc: "drop wasm-pack"
  },
  {
    commit: "9d5e5d6cfcb05219f060e68a986786d593f4e267",
    date: 1711260841,
    desc: "another test variant"
  },
  {
    commit: "c8d26e3463c036c414183ad90b876a216c06534d",
    date: 1711260730,
    desc: "avoid some double frees"
  },
  {
    commit: "0c31078e24c8f6e7f5da2a098cff5a79b75682d3",
    date: 1711258768,
    desc: "GlobalAlloc touchup"
  },
  {
    commit: "625e3de08debdb5a861569e23d7482086a170e1d",
    date: 1711257411,
    desc: "better oob"
  },
  {
    commit: "202de8081bf4b73bdb7b08669ebce76a1415cf74",
    date: 1711257402,
    desc: "more fpu"
  },
  {
    commit: "0aaa2aecb94741b05d227e25499854a09502bce8",
    date: 1711256316,
    desc: "more GetSysColor"
  },
  {
    commit: "94b62f1abbff9cd2b368c4dcf5926e26010f95c6",
    date: 1711255587,
    desc: "null brush"
  },
  {
    commit: "7e973b1fa63be622ab13c7482a04f68145532e14",
    date: 1711254787,
    desc: "more mmx ops"
  },
  {
    commit: "5ac5c013f25f0491db8b8fecb6bbaf70bbbdebd5",
    date: 1711237191,
    desc: "clc"
  },
  {
    commit: "ccbee99439945c0117473ac48ae8380608a00405",
    date: 1711237138,
    desc: "fscale"
  },
  {
    commit: "20f60b0858d0f031b4685687e57afe1865b79de5",
    date: 1711236471,
    desc: "another xor"
  },
  {
    commit: "d0a5faa3dbf835a0b8aba8e839eec5e348096cc1",
    date: 1711236372,
    desc: "CreateThread hack"
  },
  {
    commit: "de22d6e8b60902ca22eaa3b4b3a25b1f5e6a8639",
    date: 1711214261,
    desc: "more error handling to not crash on crinkler output"
  },
  {
    commit: "3112d2a270aa573456c40382f8e832ccc217d618",
    date: 1711165844,
    desc: "refactor relocations"
  },
  {
    commit: "e160f3e1f997e8f29152992c8c22ff8e1a31c58b",
    date: 1711140874,
    desc: "move snapshotting out a layer"
  },
  {
    commit: "e8d4d7a0dfbb6c6936e7c734630371b747f960df",
    date: 1710611890,
    desc: "fix debugger button"
  },
  {
    commit: "8e89924f29945cb10288d81d593a53662dc0b4e4",
    date: 1710610638,
    desc: "some more fpu"
  },
  {
    commit: "b90b05de0a64800da2e63b8ee7b9a78f6047c805",
    date: 1710608493,
    desc: "yet more fpu bugs :("
  },
  {
    commit: "7210594f9d632821bbab5303c6b73560c0e80031",
    date: 1710608493,
    desc: "another fsub"
  },
  {
    commit: "d0da8a334ccfc4d4ed6c5f938533eecf413627d9",
    date: 1710608493,
    desc: "allow some more todo bitmaps"
  },
  {
    commit: "f18fe56d3e86af044e368cf4fc452c7b836f6730",
    date: 1710524105,
    desc: "wndclass background color"
  },
  {
    commit: "b33224fa6914ab83ba9696f9bf09d218c935a7b4",
    date: 1710524072,
    desc: "finish fp regs list"
  },
  {
    commit: "46d81905fb1508bef30a649fff230494eb9f8d5a",
    date: 1710459447,
    desc: "less wildcard imports"
  },
  {
    commit: "28475ccbcce7c028e89332b6d3da8a0b3e31bb6b",
    date: 1710440972,
    desc: "move reader"
  },
  {
    commit: "0d32b695fe855b853aabccdb83ddaebed858dd5d",
    date: 1710359934,
    desc: "simpler"
  },
  {
    commit: "d374ab7c243d39032c241d10ba9482b5f135b7d6",
    date: 1710359900,
    desc: "drop unused"
  },
  {
    commit: "8b97e37458c8acccb31afbfe9d1476340cb3f0cb",
    date: 1710357249,
    desc: "simpler expect"
  },
  {
    commit: "50b5db7cc6b8d79fe55306dfd9084c8c9cb50d52",
    date: 1710356681,
    desc: "convert reader to slices"
  },
  {
    commit: "6beed647a45a598ed1cd8734a64571f8e8216d2a",
    date: 1710355640,
    desc: "convert imports to slices, much simpler iterators"
  },
  {
    commit: "09724fb29a415bb543221fa197204ae72c5b6ebf",
    date: 1710354672,
    desc: "use slice in find_resource"
  },
  {
    commit: "684c8c6f0897c496a4cb6c6fc168a16373131c02",
    date: 1710354451,
    desc: "pe resources to slices"
  },
  {
    commit: "b4f6f87294a012a101c6a83abe3557fee0196d69",
    date: 1710353501,
    desc: "avoid view_n in favor of iterating Pod copy"
  },
  {
    commit: "c9c48d284142fb28bc5d8ac2ded9174a28b555e5",
    date: 1710351829,
    desc: "pe imports to slices"
  },
  {
    commit: "d6d1bddbb901bda5ac306d0fb7f46817a77d2734",
    date: 1710351410,
    desc: "start migrating pe to slices"
  },
  {
    commit: "aedccbe4dce11cb57c456c971afb2d5837e35d65",
    date: 1710348518,
    desc: "rename .get to mention pod"
  },
  {
    commit: "f2cd496a43288f953de1b7994152fcc65a4c2c02",
    date: 1710347680,
    desc: "more into extensions"
  },
  {
    commit: "ead028719729d1bd1af768061de316c919a80eae",
    date: 1710346997,
    desc: "move some of mem into an Extensions trait"
  },
  {
    commit: "78dcdee7209218fcb60121c5d2f9cde55a33f954",
    date: 1710273799,
    desc: "note on mem"
  },
  {
    commit: "823fcb98bce36f7a50a998bdc33648e24ebdc4cf",
    date: 1710273757,
    desc: "reduce str-mangling api"
  },
  {
    commit: "05252fb6469344c3a3aed0f3dae1bcee171ba504",
    date: 1710273649,
    desc: "move str16"
  },
  {
    commit: "4a0fd34efde09e7e05dbc89609fcb79e0620ad31",
    date: 1710262861,
    desc: "remove memory resize in favor of preallocating"
  },
  {
    commit: "b0a8e6222c297da04d1742c78821bce25b95576a",
    date: 1710261983,
    desc: "move memory into emu"
  }
];

// node_modules/d3-array/src/ascending.js
function ascending(a, b) {
  return a == null || b == null ? NaN : a < b ? -1 : a > b ? 1 : a >= b ? 0 : NaN;
}

// node_modules/d3-array/src/descending.js
function descending(a, b) {
  return a == null || b == null ? NaN : b < a ? -1 : b > a ? 1 : b >= a ? 0 : NaN;
}

// node_modules/d3-array/src/bisector.js
function bisector(f) {
  let compare1, compare2, delta;
  if (f.length !== 2) {
    compare1 = ascending;
    compare2 = (d, x2) => ascending(f(d), x2);
    delta = (d, x2) => f(d) - x2;
  } else {
    compare1 = f === ascending || f === descending ? f : zero;
    compare2 = f;
    delta = f;
  }
  function left2(a, x2, lo = 0, hi = a.length) {
    if (lo < hi) {
      if (compare1(x2, x2) !== 0) return hi;
      do {
        const mid = lo + hi >>> 1;
        if (compare2(a[mid], x2) < 0) lo = mid + 1;
        else hi = mid;
      } while (lo < hi);
    }
    return lo;
  }
  function right2(a, x2, lo = 0, hi = a.length) {
    if (lo < hi) {
      if (compare1(x2, x2) !== 0) return hi;
      do {
        const mid = lo + hi >>> 1;
        if (compare2(a[mid], x2) <= 0) lo = mid + 1;
        else hi = mid;
      } while (lo < hi);
    }
    return lo;
  }
  function center2(a, x2, lo = 0, hi = a.length) {
    const i = left2(a, x2, lo, hi - 1);
    return i > lo && delta(a[i - 1], x2) > -delta(a[i], x2) ? i - 1 : i;
  }
  return { left: left2, center: center2, right: right2 };
}
function zero() {
  return 0;
}

// node_modules/d3-array/src/number.js
function number(x2) {
  return x2 === null ? NaN : +x2;
}

// node_modules/d3-array/src/bisect.js
var ascendingBisect = bisector(ascending);
var bisectRight = ascendingBisect.right;
var bisectLeft = ascendingBisect.left;
var bisectCenter = bisector(number).center;
var bisect_default = bisectRight;

// node_modules/d3-array/src/extent.js
function extent(values, valueof) {
  let min2;
  let max2;
  if (valueof === void 0) {
    for (const value of values) {
      if (value != null) {
        if (min2 === void 0) {
          if (value >= value) min2 = max2 = value;
        } else {
          if (min2 > value) min2 = value;
          if (max2 < value) max2 = value;
        }
      }
    }
  } else {
    let index = -1;
    for (let value of values) {
      if ((value = valueof(value, ++index, values)) != null) {
        if (min2 === void 0) {
          if (value >= value) min2 = max2 = value;
        } else {
          if (min2 > value) min2 = value;
          if (max2 < value) max2 = value;
        }
      }
    }
  }
  return [min2, max2];
}

// node_modules/d3-array/src/ticks.js
var e10 = Math.sqrt(50);
var e5 = Math.sqrt(10);
var e2 = Math.sqrt(2);
function tickSpec(start2, stop, count) {
  const step = (stop - start2) / Math.max(0, count), power = Math.floor(Math.log10(step)), error = step / Math.pow(10, power), factor = error >= e10 ? 10 : error >= e5 ? 5 : error >= e2 ? 2 : 1;
  let i1, i2, inc;
  if (power < 0) {
    inc = Math.pow(10, -power) / factor;
    i1 = Math.round(start2 * inc);
    i2 = Math.round(stop * inc);
    if (i1 / inc < start2) ++i1;
    if (i2 / inc > stop) --i2;
    inc = -inc;
  } else {
    inc = Math.pow(10, power) * factor;
    i1 = Math.round(start2 / inc);
    i2 = Math.round(stop / inc);
    if (i1 * inc < start2) ++i1;
    if (i2 * inc > stop) --i2;
  }
  if (i2 < i1 && 0.5 <= count && count < 2) return tickSpec(start2, stop, count * 2);
  return [i1, i2, inc];
}
function ticks(start2, stop, count) {
  stop = +stop, start2 = +start2, count = +count;
  if (!(count > 0)) return [];
  if (start2 === stop) return [start2];
  const reverse = stop < start2, [i1, i2, inc] = reverse ? tickSpec(stop, start2, count) : tickSpec(start2, stop, count);
  if (!(i2 >= i1)) return [];
  const n = i2 - i1 + 1, ticks2 = new Array(n);
  if (reverse) {
    if (inc < 0) for (let i = 0; i < n; ++i) ticks2[i] = (i2 - i) / -inc;
    else for (let i = 0; i < n; ++i) ticks2[i] = (i2 - i) * inc;
  } else {
    if (inc < 0) for (let i = 0; i < n; ++i) ticks2[i] = (i1 + i) / -inc;
    else for (let i = 0; i < n; ++i) ticks2[i] = (i1 + i) * inc;
  }
  return ticks2;
}
function tickIncrement(start2, stop, count) {
  stop = +stop, start2 = +start2, count = +count;
  return tickSpec(start2, stop, count)[2];
}
function tickStep(start2, stop, count) {
  stop = +stop, start2 = +start2, count = +count;
  const reverse = stop < start2, inc = reverse ? tickIncrement(stop, start2, count) : tickIncrement(start2, stop, count);
  return (reverse ? -1 : 1) * (inc < 0 ? 1 / -inc : inc);
}

// node_modules/d3-axis/src/identity.js
function identity_default(x2) {
  return x2;
}

// node_modules/d3-axis/src/axis.js
var top = 1;
var right = 2;
var bottom = 3;
var left = 4;
var epsilon = 1e-6;
function translateX(x2) {
  return "translate(" + x2 + ",0)";
}
function translateY(y2) {
  return "translate(0," + y2 + ")";
}
function number2(scale) {
  return (d) => +scale(d);
}
function center(scale, offset) {
  offset = Math.max(0, scale.bandwidth() - offset * 2) / 2;
  if (scale.round()) offset = Math.round(offset);
  return (d) => +scale(d) + offset;
}
function entering() {
  return !this.__axis;
}
function axis(orient, scale) {
  var tickArguments = [], tickValues = null, tickFormat2 = null, tickSizeInner = 6, tickSizeOuter = 6, tickPadding = 3, offset = typeof window !== "undefined" && window.devicePixelRatio > 1 ? 0 : 0.5, k = orient === top || orient === left ? -1 : 1, x2 = orient === left || orient === right ? "x" : "y", transform2 = orient === top || orient === bottom ? translateX : translateY;
  function axis2(context) {
    var values = tickValues == null ? scale.ticks ? scale.ticks.apply(scale, tickArguments) : scale.domain() : tickValues, format2 = tickFormat2 == null ? scale.tickFormat ? scale.tickFormat.apply(scale, tickArguments) : identity_default : tickFormat2, spacing = Math.max(tickSizeInner, 0) + tickPadding, range = scale.range(), range0 = +range[0] + offset, range1 = +range[range.length - 1] + offset, position = (scale.bandwidth ? center : number2)(scale.copy(), offset), selection2 = context.selection ? context.selection() : context, path2 = selection2.selectAll(".domain").data([null]), tick = selection2.selectAll(".tick").data(values, scale).order(), tickExit = tick.exit(), tickEnter = tick.enter().append("g").attr("class", "tick"), line = tick.select("line"), text = tick.select("text");
    path2 = path2.merge(path2.enter().insert("path", ".tick").attr("class", "domain").attr("stroke", "currentColor"));
    tick = tick.merge(tickEnter);
    line = line.merge(tickEnter.append("line").attr("stroke", "currentColor").attr(x2 + "2", k * tickSizeInner));
    text = text.merge(tickEnter.append("text").attr("fill", "currentColor").attr(x2, k * spacing).attr("dy", orient === top ? "0em" : orient === bottom ? "0.71em" : "0.32em"));
    if (context !== selection2) {
      path2 = path2.transition(context);
      tick = tick.transition(context);
      line = line.transition(context);
      text = text.transition(context);
      tickExit = tickExit.transition(context).attr("opacity", epsilon).attr("transform", function(d) {
        return isFinite(d = position(d)) ? transform2(d + offset) : this.getAttribute("transform");
      });
      tickEnter.attr("opacity", epsilon).attr("transform", function(d) {
        var p = this.parentNode.__axis;
        return transform2((p && isFinite(p = p(d)) ? p : position(d)) + offset);
      });
    }
    tickExit.remove();
    path2.attr("d", orient === left || orient === right ? tickSizeOuter ? "M" + k * tickSizeOuter + "," + range0 + "H" + offset + "V" + range1 + "H" + k * tickSizeOuter : "M" + offset + "," + range0 + "V" + range1 : tickSizeOuter ? "M" + range0 + "," + k * tickSizeOuter + "V" + offset + "H" + range1 + "V" + k * tickSizeOuter : "M" + range0 + "," + offset + "H" + range1);
    tick.attr("opacity", 1).attr("transform", function(d) {
      return transform2(position(d) + offset);
    });
    line.attr(x2 + "2", k * tickSizeInner);
    text.attr(x2, k * spacing).text(format2);
    selection2.filter(entering).attr("fill", "none").attr("font-size", 10).attr("font-family", "sans-serif").attr("text-anchor", orient === right ? "start" : orient === left ? "end" : "middle");
    selection2.each(function() {
      this.__axis = position;
    });
  }
  axis2.scale = function(_) {
    return arguments.length ? (scale = _, axis2) : scale;
  };
  axis2.ticks = function() {
    return tickArguments = Array.from(arguments), axis2;
  };
  axis2.tickArguments = function(_) {
    return arguments.length ? (tickArguments = _ == null ? [] : Array.from(_), axis2) : tickArguments.slice();
  };
  axis2.tickValues = function(_) {
    return arguments.length ? (tickValues = _ == null ? null : Array.from(_), axis2) : tickValues && tickValues.slice();
  };
  axis2.tickFormat = function(_) {
    return arguments.length ? (tickFormat2 = _, axis2) : tickFormat2;
  };
  axis2.tickSize = function(_) {
    return arguments.length ? (tickSizeInner = tickSizeOuter = +_, axis2) : tickSizeInner;
  };
  axis2.tickSizeInner = function(_) {
    return arguments.length ? (tickSizeInner = +_, axis2) : tickSizeInner;
  };
  axis2.tickSizeOuter = function(_) {
    return arguments.length ? (tickSizeOuter = +_, axis2) : tickSizeOuter;
  };
  axis2.tickPadding = function(_) {
    return arguments.length ? (tickPadding = +_, axis2) : tickPadding;
  };
  axis2.offset = function(_) {
    return arguments.length ? (offset = +_, axis2) : offset;
  };
  return axis2;
}
function axisBottom(scale) {
  return axis(bottom, scale);
}
function axisLeft(scale) {
  return axis(left, scale);
}

// node_modules/d3-dispatch/src/dispatch.js
var noop = { value: () => {
} };
function dispatch() {
  for (var i = 0, n = arguments.length, _ = {}, t; i < n; ++i) {
    if (!(t = arguments[i] + "") || t in _ || /[\s.]/.test(t)) throw new Error("illegal type: " + t);
    _[t] = [];
  }
  return new Dispatch(_);
}
function Dispatch(_) {
  this._ = _;
}
function parseTypenames(typenames, types) {
  return typenames.trim().split(/^|\s+/).map(function(t) {
    var name = "", i = t.indexOf(".");
    if (i >= 0) name = t.slice(i + 1), t = t.slice(0, i);
    if (t && !types.hasOwnProperty(t)) throw new Error("unknown type: " + t);
    return { type: t, name };
  });
}
Dispatch.prototype = dispatch.prototype = {
  constructor: Dispatch,
  on: function(typename, callback) {
    var _ = this._, T = parseTypenames(typename + "", _), t, i = -1, n = T.length;
    if (arguments.length < 2) {
      while (++i < n) if ((t = (typename = T[i]).type) && (t = get(_[t], typename.name))) return t;
      return;
    }
    if (callback != null && typeof callback !== "function") throw new Error("invalid callback: " + callback);
    while (++i < n) {
      if (t = (typename = T[i]).type) _[t] = set(_[t], typename.name, callback);
      else if (callback == null) for (t in _) _[t] = set(_[t], typename.name, null);
    }
    return this;
  },
  copy: function() {
    var copy2 = {}, _ = this._;
    for (var t in _) copy2[t] = _[t].slice();
    return new Dispatch(copy2);
  },
  call: function(type2, that) {
    if ((n = arguments.length - 2) > 0) for (var args = new Array(n), i = 0, n, t; i < n; ++i) args[i] = arguments[i + 2];
    if (!this._.hasOwnProperty(type2)) throw new Error("unknown type: " + type2);
    for (t = this._[type2], i = 0, n = t.length; i < n; ++i) t[i].value.apply(that, args);
  },
  apply: function(type2, that, args) {
    if (!this._.hasOwnProperty(type2)) throw new Error("unknown type: " + type2);
    for (var t = this._[type2], i = 0, n = t.length; i < n; ++i) t[i].value.apply(that, args);
  }
};
function get(type2, name) {
  for (var i = 0, n = type2.length, c; i < n; ++i) {
    if ((c = type2[i]).name === name) {
      return c.value;
    }
  }
}
function set(type2, name, callback) {
  for (var i = 0, n = type2.length; i < n; ++i) {
    if (type2[i].name === name) {
      type2[i] = noop, type2 = type2.slice(0, i).concat(type2.slice(i + 1));
      break;
    }
  }
  if (callback != null) type2.push({ name, value: callback });
  return type2;
}
var dispatch_default = dispatch;

// node_modules/d3-selection/src/namespaces.js
var xhtml = "http://www.w3.org/1999/xhtml";
var namespaces_default = {
  svg: "http://www.w3.org/2000/svg",
  xhtml,
  xlink: "http://www.w3.org/1999/xlink",
  xml: "http://www.w3.org/XML/1998/namespace",
  xmlns: "http://www.w3.org/2000/xmlns/"
};

// node_modules/d3-selection/src/namespace.js
function namespace_default(name) {
  var prefix = name += "", i = prefix.indexOf(":");
  if (i >= 0 && (prefix = name.slice(0, i)) !== "xmlns") name = name.slice(i + 1);
  return namespaces_default.hasOwnProperty(prefix) ? { space: namespaces_default[prefix], local: name } : name;
}

// node_modules/d3-selection/src/creator.js
function creatorInherit(name) {
  return function() {
    var document2 = this.ownerDocument, uri = this.namespaceURI;
    return uri === xhtml && document2.documentElement.namespaceURI === xhtml ? document2.createElement(name) : document2.createElementNS(uri, name);
  };
}
function creatorFixed(fullname) {
  return function() {
    return this.ownerDocument.createElementNS(fullname.space, fullname.local);
  };
}
function creator_default(name) {
  var fullname = namespace_default(name);
  return (fullname.local ? creatorFixed : creatorInherit)(fullname);
}

// node_modules/d3-selection/src/selector.js
function none() {
}
function selector_default(selector) {
  return selector == null ? none : function() {
    return this.querySelector(selector);
  };
}

// node_modules/d3-selection/src/selection/select.js
function select_default(select) {
  if (typeof select !== "function") select = selector_default(select);
  for (var groups = this._groups, m = groups.length, subgroups = new Array(m), j = 0; j < m; ++j) {
    for (var group = groups[j], n = group.length, subgroup = subgroups[j] = new Array(n), node, subnode, i = 0; i < n; ++i) {
      if ((node = group[i]) && (subnode = select.call(node, node.__data__, i, group))) {
        if ("__data__" in node) subnode.__data__ = node.__data__;
        subgroup[i] = subnode;
      }
    }
  }
  return new Selection(subgroups, this._parents);
}

// node_modules/d3-selection/src/array.js
function array(x2) {
  return x2 == null ? [] : Array.isArray(x2) ? x2 : Array.from(x2);
}

// node_modules/d3-selection/src/selectorAll.js
function empty() {
  return [];
}
function selectorAll_default(selector) {
  return selector == null ? empty : function() {
    return this.querySelectorAll(selector);
  };
}

// node_modules/d3-selection/src/selection/selectAll.js
function arrayAll(select) {
  return function() {
    return array(select.apply(this, arguments));
  };
}
function selectAll_default(select) {
  if (typeof select === "function") select = arrayAll(select);
  else select = selectorAll_default(select);
  for (var groups = this._groups, m = groups.length, subgroups = [], parents = [], j = 0; j < m; ++j) {
    for (var group = groups[j], n = group.length, node, i = 0; i < n; ++i) {
      if (node = group[i]) {
        subgroups.push(select.call(node, node.__data__, i, group));
        parents.push(node);
      }
    }
  }
  return new Selection(subgroups, parents);
}

// node_modules/d3-selection/src/matcher.js
function matcher_default(selector) {
  return function() {
    return this.matches(selector);
  };
}
function childMatcher(selector) {
  return function(node) {
    return node.matches(selector);
  };
}

// node_modules/d3-selection/src/selection/selectChild.js
var find = Array.prototype.find;
function childFind(match) {
  return function() {
    return find.call(this.children, match);
  };
}
function childFirst() {
  return this.firstElementChild;
}
function selectChild_default(match) {
  return this.select(match == null ? childFirst : childFind(typeof match === "function" ? match : childMatcher(match)));
}

// node_modules/d3-selection/src/selection/selectChildren.js
var filter = Array.prototype.filter;
function children() {
  return Array.from(this.children);
}
function childrenFilter(match) {
  return function() {
    return filter.call(this.children, match);
  };
}
function selectChildren_default(match) {
  return this.selectAll(match == null ? children : childrenFilter(typeof match === "function" ? match : childMatcher(match)));
}

// node_modules/d3-selection/src/selection/filter.js
function filter_default(match) {
  if (typeof match !== "function") match = matcher_default(match);
  for (var groups = this._groups, m = groups.length, subgroups = new Array(m), j = 0; j < m; ++j) {
    for (var group = groups[j], n = group.length, subgroup = subgroups[j] = [], node, i = 0; i < n; ++i) {
      if ((node = group[i]) && match.call(node, node.__data__, i, group)) {
        subgroup.push(node);
      }
    }
  }
  return new Selection(subgroups, this._parents);
}

// node_modules/d3-selection/src/selection/sparse.js
function sparse_default(update) {
  return new Array(update.length);
}

// node_modules/d3-selection/src/selection/enter.js
function enter_default() {
  return new Selection(this._enter || this._groups.map(sparse_default), this._parents);
}
function EnterNode(parent, datum2) {
  this.ownerDocument = parent.ownerDocument;
  this.namespaceURI = parent.namespaceURI;
  this._next = null;
  this._parent = parent;
  this.__data__ = datum2;
}
EnterNode.prototype = {
  constructor: EnterNode,
  appendChild: function(child) {
    return this._parent.insertBefore(child, this._next);
  },
  insertBefore: function(child, next) {
    return this._parent.insertBefore(child, next);
  },
  querySelector: function(selector) {
    return this._parent.querySelector(selector);
  },
  querySelectorAll: function(selector) {
    return this._parent.querySelectorAll(selector);
  }
};

// node_modules/d3-selection/src/constant.js
function constant_default(x2) {
  return function() {
    return x2;
  };
}

// node_modules/d3-selection/src/selection/data.js
function bindIndex(parent, group, enter, update, exit, data) {
  var i = 0, node, groupLength = group.length, dataLength = data.length;
  for (; i < dataLength; ++i) {
    if (node = group[i]) {
      node.__data__ = data[i];
      update[i] = node;
    } else {
      enter[i] = new EnterNode(parent, data[i]);
    }
  }
  for (; i < groupLength; ++i) {
    if (node = group[i]) {
      exit[i] = node;
    }
  }
}
function bindKey(parent, group, enter, update, exit, data, key) {
  var i, node, nodeByKeyValue = /* @__PURE__ */ new Map(), groupLength = group.length, dataLength = data.length, keyValues = new Array(groupLength), keyValue;
  for (i = 0; i < groupLength; ++i) {
    if (node = group[i]) {
      keyValues[i] = keyValue = key.call(node, node.__data__, i, group) + "";
      if (nodeByKeyValue.has(keyValue)) {
        exit[i] = node;
      } else {
        nodeByKeyValue.set(keyValue, node);
      }
    }
  }
  for (i = 0; i < dataLength; ++i) {
    keyValue = key.call(parent, data[i], i, data) + "";
    if (node = nodeByKeyValue.get(keyValue)) {
      update[i] = node;
      node.__data__ = data[i];
      nodeByKeyValue.delete(keyValue);
    } else {
      enter[i] = new EnterNode(parent, data[i]);
    }
  }
  for (i = 0; i < groupLength; ++i) {
    if ((node = group[i]) && nodeByKeyValue.get(keyValues[i]) === node) {
      exit[i] = node;
    }
  }
}
function datum(node) {
  return node.__data__;
}
function data_default(value, key) {
  if (!arguments.length) return Array.from(this, datum);
  var bind = key ? bindKey : bindIndex, parents = this._parents, groups = this._groups;
  if (typeof value !== "function") value = constant_default(value);
  for (var m = groups.length, update = new Array(m), enter = new Array(m), exit = new Array(m), j = 0; j < m; ++j) {
    var parent = parents[j], group = groups[j], groupLength = group.length, data = arraylike(value.call(parent, parent && parent.__data__, j, parents)), dataLength = data.length, enterGroup = enter[j] = new Array(dataLength), updateGroup = update[j] = new Array(dataLength), exitGroup = exit[j] = new Array(groupLength);
    bind(parent, group, enterGroup, updateGroup, exitGroup, data, key);
    for (var i0 = 0, i1 = 0, previous, next; i0 < dataLength; ++i0) {
      if (previous = enterGroup[i0]) {
        if (i0 >= i1) i1 = i0 + 1;
        while (!(next = updateGroup[i1]) && ++i1 < dataLength) ;
        previous._next = next || null;
      }
    }
  }
  update = new Selection(update, parents);
  update._enter = enter;
  update._exit = exit;
  return update;
}
function arraylike(data) {
  return typeof data === "object" && "length" in data ? data : Array.from(data);
}

// node_modules/d3-selection/src/selection/exit.js
function exit_default() {
  return new Selection(this._exit || this._groups.map(sparse_default), this._parents);
}

// node_modules/d3-selection/src/selection/join.js
function join_default(onenter, onupdate, onexit) {
  var enter = this.enter(), update = this, exit = this.exit();
  if (typeof onenter === "function") {
    enter = onenter(enter);
    if (enter) enter = enter.selection();
  } else {
    enter = enter.append(onenter + "");
  }
  if (onupdate != null) {
    update = onupdate(update);
    if (update) update = update.selection();
  }
  if (onexit == null) exit.remove();
  else onexit(exit);
  return enter && update ? enter.merge(update).order() : update;
}

// node_modules/d3-selection/src/selection/merge.js
function merge_default(context) {
  var selection2 = context.selection ? context.selection() : context;
  for (var groups0 = this._groups, groups1 = selection2._groups, m0 = groups0.length, m1 = groups1.length, m = Math.min(m0, m1), merges = new Array(m0), j = 0; j < m; ++j) {
    for (var group0 = groups0[j], group1 = groups1[j], n = group0.length, merge = merges[j] = new Array(n), node, i = 0; i < n; ++i) {
      if (node = group0[i] || group1[i]) {
        merge[i] = node;
      }
    }
  }
  for (; j < m0; ++j) {
    merges[j] = groups0[j];
  }
  return new Selection(merges, this._parents);
}

// node_modules/d3-selection/src/selection/order.js
function order_default() {
  for (var groups = this._groups, j = -1, m = groups.length; ++j < m; ) {
    for (var group = groups[j], i = group.length - 1, next = group[i], node; --i >= 0; ) {
      if (node = group[i]) {
        if (next && node.compareDocumentPosition(next) ^ 4) next.parentNode.insertBefore(node, next);
        next = node;
      }
    }
  }
  return this;
}

// node_modules/d3-selection/src/selection/sort.js
function sort_default(compare) {
  if (!compare) compare = ascending2;
  function compareNode(a, b) {
    return a && b ? compare(a.__data__, b.__data__) : !a - !b;
  }
  for (var groups = this._groups, m = groups.length, sortgroups = new Array(m), j = 0; j < m; ++j) {
    for (var group = groups[j], n = group.length, sortgroup = sortgroups[j] = new Array(n), node, i = 0; i < n; ++i) {
      if (node = group[i]) {
        sortgroup[i] = node;
      }
    }
    sortgroup.sort(compareNode);
  }
  return new Selection(sortgroups, this._parents).order();
}
function ascending2(a, b) {
  return a < b ? -1 : a > b ? 1 : a >= b ? 0 : NaN;
}

// node_modules/d3-selection/src/selection/call.js
function call_default() {
  var callback = arguments[0];
  arguments[0] = this;
  callback.apply(null, arguments);
  return this;
}

// node_modules/d3-selection/src/selection/nodes.js
function nodes_default() {
  return Array.from(this);
}

// node_modules/d3-selection/src/selection/node.js
function node_default() {
  for (var groups = this._groups, j = 0, m = groups.length; j < m; ++j) {
    for (var group = groups[j], i = 0, n = group.length; i < n; ++i) {
      var node = group[i];
      if (node) return node;
    }
  }
  return null;
}

// node_modules/d3-selection/src/selection/size.js
function size_default() {
  let size = 0;
  for (const node of this) ++size;
  return size;
}

// node_modules/d3-selection/src/selection/empty.js
function empty_default() {
  return !this.node();
}

// node_modules/d3-selection/src/selection/each.js
function each_default(callback) {
  for (var groups = this._groups, j = 0, m = groups.length; j < m; ++j) {
    for (var group = groups[j], i = 0, n = group.length, node; i < n; ++i) {
      if (node = group[i]) callback.call(node, node.__data__, i, group);
    }
  }
  return this;
}

// node_modules/d3-selection/src/selection/attr.js
function attrRemove(name) {
  return function() {
    this.removeAttribute(name);
  };
}
function attrRemoveNS(fullname) {
  return function() {
    this.removeAttributeNS(fullname.space, fullname.local);
  };
}
function attrConstant(name, value) {
  return function() {
    this.setAttribute(name, value);
  };
}
function attrConstantNS(fullname, value) {
  return function() {
    this.setAttributeNS(fullname.space, fullname.local, value);
  };
}
function attrFunction(name, value) {
  return function() {
    var v = value.apply(this, arguments);
    if (v == null) this.removeAttribute(name);
    else this.setAttribute(name, v);
  };
}
function attrFunctionNS(fullname, value) {
  return function() {
    var v = value.apply(this, arguments);
    if (v == null) this.removeAttributeNS(fullname.space, fullname.local);
    else this.setAttributeNS(fullname.space, fullname.local, v);
  };
}
function attr_default(name, value) {
  var fullname = namespace_default(name);
  if (arguments.length < 2) {
    var node = this.node();
    return fullname.local ? node.getAttributeNS(fullname.space, fullname.local) : node.getAttribute(fullname);
  }
  return this.each((value == null ? fullname.local ? attrRemoveNS : attrRemove : typeof value === "function" ? fullname.local ? attrFunctionNS : attrFunction : fullname.local ? attrConstantNS : attrConstant)(fullname, value));
}

// node_modules/d3-selection/src/window.js
function window_default(node) {
  return node.ownerDocument && node.ownerDocument.defaultView || node.document && node || node.defaultView;
}

// node_modules/d3-selection/src/selection/style.js
function styleRemove(name) {
  return function() {
    this.style.removeProperty(name);
  };
}
function styleConstant(name, value, priority) {
  return function() {
    this.style.setProperty(name, value, priority);
  };
}
function styleFunction(name, value, priority) {
  return function() {
    var v = value.apply(this, arguments);
    if (v == null) this.style.removeProperty(name);
    else this.style.setProperty(name, v, priority);
  };
}
function style_default(name, value, priority) {
  return arguments.length > 1 ? this.each((value == null ? styleRemove : typeof value === "function" ? styleFunction : styleConstant)(name, value, priority == null ? "" : priority)) : styleValue(this.node(), name);
}
function styleValue(node, name) {
  return node.style.getPropertyValue(name) || window_default(node).getComputedStyle(node, null).getPropertyValue(name);
}

// node_modules/d3-selection/src/selection/property.js
function propertyRemove(name) {
  return function() {
    delete this[name];
  };
}
function propertyConstant(name, value) {
  return function() {
    this[name] = value;
  };
}
function propertyFunction(name, value) {
  return function() {
    var v = value.apply(this, arguments);
    if (v == null) delete this[name];
    else this[name] = v;
  };
}
function property_default(name, value) {
  return arguments.length > 1 ? this.each((value == null ? propertyRemove : typeof value === "function" ? propertyFunction : propertyConstant)(name, value)) : this.node()[name];
}

// node_modules/d3-selection/src/selection/classed.js
function classArray(string) {
  return string.trim().split(/^|\s+/);
}
function classList(node) {
  return node.classList || new ClassList(node);
}
function ClassList(node) {
  this._node = node;
  this._names = classArray(node.getAttribute("class") || "");
}
ClassList.prototype = {
  add: function(name) {
    var i = this._names.indexOf(name);
    if (i < 0) {
      this._names.push(name);
      this._node.setAttribute("class", this._names.join(" "));
    }
  },
  remove: function(name) {
    var i = this._names.indexOf(name);
    if (i >= 0) {
      this._names.splice(i, 1);
      this._node.setAttribute("class", this._names.join(" "));
    }
  },
  contains: function(name) {
    return this._names.indexOf(name) >= 0;
  }
};
function classedAdd(node, names) {
  var list = classList(node), i = -1, n = names.length;
  while (++i < n) list.add(names[i]);
}
function classedRemove(node, names) {
  var list = classList(node), i = -1, n = names.length;
  while (++i < n) list.remove(names[i]);
}
function classedTrue(names) {
  return function() {
    classedAdd(this, names);
  };
}
function classedFalse(names) {
  return function() {
    classedRemove(this, names);
  };
}
function classedFunction(names, value) {
  return function() {
    (value.apply(this, arguments) ? classedAdd : classedRemove)(this, names);
  };
}
function classed_default(name, value) {
  var names = classArray(name + "");
  if (arguments.length < 2) {
    var list = classList(this.node()), i = -1, n = names.length;
    while (++i < n) if (!list.contains(names[i])) return false;
    return true;
  }
  return this.each((typeof value === "function" ? classedFunction : value ? classedTrue : classedFalse)(names, value));
}

// node_modules/d3-selection/src/selection/text.js
function textRemove() {
  this.textContent = "";
}
function textConstant(value) {
  return function() {
    this.textContent = value;
  };
}
function textFunction(value) {
  return function() {
    var v = value.apply(this, arguments);
    this.textContent = v == null ? "" : v;
  };
}
function text_default(value) {
  return arguments.length ? this.each(value == null ? textRemove : (typeof value === "function" ? textFunction : textConstant)(value)) : this.node().textContent;
}

// node_modules/d3-selection/src/selection/html.js
function htmlRemove() {
  this.innerHTML = "";
}
function htmlConstant(value) {
  return function() {
    this.innerHTML = value;
  };
}
function htmlFunction(value) {
  return function() {
    var v = value.apply(this, arguments);
    this.innerHTML = v == null ? "" : v;
  };
}
function html_default(value) {
  return arguments.length ? this.each(value == null ? htmlRemove : (typeof value === "function" ? htmlFunction : htmlConstant)(value)) : this.node().innerHTML;
}

// node_modules/d3-selection/src/selection/raise.js
function raise() {
  if (this.nextSibling) this.parentNode.appendChild(this);
}
function raise_default() {
  return this.each(raise);
}

// node_modules/d3-selection/src/selection/lower.js
function lower() {
  if (this.previousSibling) this.parentNode.insertBefore(this, this.parentNode.firstChild);
}
function lower_default() {
  return this.each(lower);
}

// node_modules/d3-selection/src/selection/append.js
function append_default(name) {
  var create2 = typeof name === "function" ? name : creator_default(name);
  return this.select(function() {
    return this.appendChild(create2.apply(this, arguments));
  });
}

// node_modules/d3-selection/src/selection/insert.js
function constantNull() {
  return null;
}
function insert_default(name, before) {
  var create2 = typeof name === "function" ? name : creator_default(name), select = before == null ? constantNull : typeof before === "function" ? before : selector_default(before);
  return this.select(function() {
    return this.insertBefore(create2.apply(this, arguments), select.apply(this, arguments) || null);
  });
}

// node_modules/d3-selection/src/selection/remove.js
function remove() {
  var parent = this.parentNode;
  if (parent) parent.removeChild(this);
}
function remove_default() {
  return this.each(remove);
}

// node_modules/d3-selection/src/selection/clone.js
function selection_cloneShallow() {
  var clone = this.cloneNode(false), parent = this.parentNode;
  return parent ? parent.insertBefore(clone, this.nextSibling) : clone;
}
function selection_cloneDeep() {
  var clone = this.cloneNode(true), parent = this.parentNode;
  return parent ? parent.insertBefore(clone, this.nextSibling) : clone;
}
function clone_default(deep) {
  return this.select(deep ? selection_cloneDeep : selection_cloneShallow);
}

// node_modules/d3-selection/src/selection/datum.js
function datum_default(value) {
  return arguments.length ? this.property("__data__", value) : this.node().__data__;
}

// node_modules/d3-selection/src/selection/on.js
function contextListener(listener) {
  return function(event) {
    listener.call(this, event, this.__data__);
  };
}
function parseTypenames2(typenames) {
  return typenames.trim().split(/^|\s+/).map(function(t) {
    var name = "", i = t.indexOf(".");
    if (i >= 0) name = t.slice(i + 1), t = t.slice(0, i);
    return { type: t, name };
  });
}
function onRemove(typename) {
  return function() {
    var on = this.__on;
    if (!on) return;
    for (var j = 0, i = -1, m = on.length, o; j < m; ++j) {
      if (o = on[j], (!typename.type || o.type === typename.type) && o.name === typename.name) {
        this.removeEventListener(o.type, o.listener, o.options);
      } else {
        on[++i] = o;
      }
    }
    if (++i) on.length = i;
    else delete this.__on;
  };
}
function onAdd(typename, value, options) {
  return function() {
    var on = this.__on, o, listener = contextListener(value);
    if (on) for (var j = 0, m = on.length; j < m; ++j) {
      if ((o = on[j]).type === typename.type && o.name === typename.name) {
        this.removeEventListener(o.type, o.listener, o.options);
        this.addEventListener(o.type, o.listener = listener, o.options = options);
        o.value = value;
        return;
      }
    }
    this.addEventListener(typename.type, listener, options);
    o = { type: typename.type, name: typename.name, value, listener, options };
    if (!on) this.__on = [o];
    else on.push(o);
  };
}
function on_default(typename, value, options) {
  var typenames = parseTypenames2(typename + ""), i, n = typenames.length, t;
  if (arguments.length < 2) {
    var on = this.node().__on;
    if (on) for (var j = 0, m = on.length, o; j < m; ++j) {
      for (i = 0, o = on[j]; i < n; ++i) {
        if ((t = typenames[i]).type === o.type && t.name === o.name) {
          return o.value;
        }
      }
    }
    return;
  }
  on = value ? onAdd : onRemove;
  for (i = 0; i < n; ++i) this.each(on(typenames[i], value, options));
  return this;
}

// node_modules/d3-selection/src/selection/dispatch.js
function dispatchEvent(node, type2, params) {
  var window2 = window_default(node), event = window2.CustomEvent;
  if (typeof event === "function") {
    event = new event(type2, params);
  } else {
    event = window2.document.createEvent("Event");
    if (params) event.initEvent(type2, params.bubbles, params.cancelable), event.detail = params.detail;
    else event.initEvent(type2, false, false);
  }
  node.dispatchEvent(event);
}
function dispatchConstant(type2, params) {
  return function() {
    return dispatchEvent(this, type2, params);
  };
}
function dispatchFunction(type2, params) {
  return function() {
    return dispatchEvent(this, type2, params.apply(this, arguments));
  };
}
function dispatch_default2(type2, params) {
  return this.each((typeof params === "function" ? dispatchFunction : dispatchConstant)(type2, params));
}

// node_modules/d3-selection/src/selection/iterator.js
function* iterator_default() {
  for (var groups = this._groups, j = 0, m = groups.length; j < m; ++j) {
    for (var group = groups[j], i = 0, n = group.length, node; i < n; ++i) {
      if (node = group[i]) yield node;
    }
  }
}

// node_modules/d3-selection/src/selection/index.js
var root = [null];
function Selection(groups, parents) {
  this._groups = groups;
  this._parents = parents;
}
function selection() {
  return new Selection([[document.documentElement]], root);
}
function selection_selection() {
  return this;
}
Selection.prototype = selection.prototype = {
  constructor: Selection,
  select: select_default,
  selectAll: selectAll_default,
  selectChild: selectChild_default,
  selectChildren: selectChildren_default,
  filter: filter_default,
  data: data_default,
  enter: enter_default,
  exit: exit_default,
  join: join_default,
  merge: merge_default,
  selection: selection_selection,
  order: order_default,
  sort: sort_default,
  call: call_default,
  nodes: nodes_default,
  node: node_default,
  size: size_default,
  empty: empty_default,
  each: each_default,
  attr: attr_default,
  style: style_default,
  property: property_default,
  classed: classed_default,
  text: text_default,
  html: html_default,
  raise: raise_default,
  lower: lower_default,
  append: append_default,
  insert: insert_default,
  remove: remove_default,
  clone: clone_default,
  datum: datum_default,
  on: on_default,
  dispatch: dispatch_default2,
  [Symbol.iterator]: iterator_default
};
var selection_default = selection;

// node_modules/d3-selection/src/select.js
function select_default2(selector) {
  return typeof selector === "string" ? new Selection([[document.querySelector(selector)]], [document.documentElement]) : new Selection([[selector]], root);
}

// node_modules/d3-selection/src/create.js
function create_default(name) {
  return select_default2(creator_default(name).call(document.documentElement));
}

// node_modules/d3-selection/src/sourceEvent.js
function sourceEvent_default(event) {
  let sourceEvent;
  while (sourceEvent = event.sourceEvent) event = sourceEvent;
  return event;
}

// node_modules/d3-selection/src/pointer.js
function pointer_default(event, node) {
  event = sourceEvent_default(event);
  if (node === void 0) node = event.currentTarget;
  if (node) {
    var svg = node.ownerSVGElement || node;
    if (svg.createSVGPoint) {
      var point = svg.createSVGPoint();
      point.x = event.clientX, point.y = event.clientY;
      point = point.matrixTransform(node.getScreenCTM().inverse());
      return [point.x, point.y];
    }
    if (node.getBoundingClientRect) {
      var rect = node.getBoundingClientRect();
      return [event.clientX - rect.left - node.clientLeft, event.clientY - rect.top - node.clientTop];
    }
  }
  return [event.pageX, event.pageY];
}

// node_modules/d3-drag/src/noevent.js
var nonpassivecapture = { capture: true, passive: false };
function noevent_default(event) {
  event.preventDefault();
  event.stopImmediatePropagation();
}

// node_modules/d3-drag/src/nodrag.js
function nodrag_default(view) {
  var root2 = view.document.documentElement, selection2 = select_default2(view).on("dragstart.drag", noevent_default, nonpassivecapture);
  if ("onselectstart" in root2) {
    selection2.on("selectstart.drag", noevent_default, nonpassivecapture);
  } else {
    root2.__noselect = root2.style.MozUserSelect;
    root2.style.MozUserSelect = "none";
  }
}
function yesdrag(view, noclick) {
  var root2 = view.document.documentElement, selection2 = select_default2(view).on("dragstart.drag", null);
  if (noclick) {
    selection2.on("click.drag", noevent_default, nonpassivecapture);
    setTimeout(function() {
      selection2.on("click.drag", null);
    }, 0);
  }
  if ("onselectstart" in root2) {
    selection2.on("selectstart.drag", null);
  } else {
    root2.style.MozUserSelect = root2.__noselect;
    delete root2.__noselect;
  }
}

// node_modules/d3-color/src/define.js
function define_default(constructor, factory, prototype) {
  constructor.prototype = factory.prototype = prototype;
  prototype.constructor = constructor;
}
function extend(parent, definition) {
  var prototype = Object.create(parent.prototype);
  for (var key in definition) prototype[key] = definition[key];
  return prototype;
}

// node_modules/d3-color/src/color.js
function Color() {
}
var darker = 0.7;
var brighter = 1 / darker;
var reI = "\\s*([+-]?\\d+)\\s*";
var reN = "\\s*([+-]?(?:\\d*\\.)?\\d+(?:[eE][+-]?\\d+)?)\\s*";
var reP = "\\s*([+-]?(?:\\d*\\.)?\\d+(?:[eE][+-]?\\d+)?)%\\s*";
var reHex = /^#([0-9a-f]{3,8})$/;
var reRgbInteger = new RegExp(`^rgb\\(${reI},${reI},${reI}\\)$`);
var reRgbPercent = new RegExp(`^rgb\\(${reP},${reP},${reP}\\)$`);
var reRgbaInteger = new RegExp(`^rgba\\(${reI},${reI},${reI},${reN}\\)$`);
var reRgbaPercent = new RegExp(`^rgba\\(${reP},${reP},${reP},${reN}\\)$`);
var reHslPercent = new RegExp(`^hsl\\(${reN},${reP},${reP}\\)$`);
var reHslaPercent = new RegExp(`^hsla\\(${reN},${reP},${reP},${reN}\\)$`);
var named = {
  aliceblue: 15792383,
  antiquewhite: 16444375,
  aqua: 65535,
  aquamarine: 8388564,
  azure: 15794175,
  beige: 16119260,
  bisque: 16770244,
  black: 0,
  blanchedalmond: 16772045,
  blue: 255,
  blueviolet: 9055202,
  brown: 10824234,
  burlywood: 14596231,
  cadetblue: 6266528,
  chartreuse: 8388352,
  chocolate: 13789470,
  coral: 16744272,
  cornflowerblue: 6591981,
  cornsilk: 16775388,
  crimson: 14423100,
  cyan: 65535,
  darkblue: 139,
  darkcyan: 35723,
  darkgoldenrod: 12092939,
  darkgray: 11119017,
  darkgreen: 25600,
  darkgrey: 11119017,
  darkkhaki: 12433259,
  darkmagenta: 9109643,
  darkolivegreen: 5597999,
  darkorange: 16747520,
  darkorchid: 10040012,
  darkred: 9109504,
  darksalmon: 15308410,
  darkseagreen: 9419919,
  darkslateblue: 4734347,
  darkslategray: 3100495,
  darkslategrey: 3100495,
  darkturquoise: 52945,
  darkviolet: 9699539,
  deeppink: 16716947,
  deepskyblue: 49151,
  dimgray: 6908265,
  dimgrey: 6908265,
  dodgerblue: 2003199,
  firebrick: 11674146,
  floralwhite: 16775920,
  forestgreen: 2263842,
  fuchsia: 16711935,
  gainsboro: 14474460,
  ghostwhite: 16316671,
  gold: 16766720,
  goldenrod: 14329120,
  gray: 8421504,
  green: 32768,
  greenyellow: 11403055,
  grey: 8421504,
  honeydew: 15794160,
  hotpink: 16738740,
  indianred: 13458524,
  indigo: 4915330,
  ivory: 16777200,
  khaki: 15787660,
  lavender: 15132410,
  lavenderblush: 16773365,
  lawngreen: 8190976,
  lemonchiffon: 16775885,
  lightblue: 11393254,
  lightcoral: 15761536,
  lightcyan: 14745599,
  lightgoldenrodyellow: 16448210,
  lightgray: 13882323,
  lightgreen: 9498256,
  lightgrey: 13882323,
  lightpink: 16758465,
  lightsalmon: 16752762,
  lightseagreen: 2142890,
  lightskyblue: 8900346,
  lightslategray: 7833753,
  lightslategrey: 7833753,
  lightsteelblue: 11584734,
  lightyellow: 16777184,
  lime: 65280,
  limegreen: 3329330,
  linen: 16445670,
  magenta: 16711935,
  maroon: 8388608,
  mediumaquamarine: 6737322,
  mediumblue: 205,
  mediumorchid: 12211667,
  mediumpurple: 9662683,
  mediumseagreen: 3978097,
  mediumslateblue: 8087790,
  mediumspringgreen: 64154,
  mediumturquoise: 4772300,
  mediumvioletred: 13047173,
  midnightblue: 1644912,
  mintcream: 16121850,
  mistyrose: 16770273,
  moccasin: 16770229,
  navajowhite: 16768685,
  navy: 128,
  oldlace: 16643558,
  olive: 8421376,
  olivedrab: 7048739,
  orange: 16753920,
  orangered: 16729344,
  orchid: 14315734,
  palegoldenrod: 15657130,
  palegreen: 10025880,
  paleturquoise: 11529966,
  palevioletred: 14381203,
  papayawhip: 16773077,
  peachpuff: 16767673,
  peru: 13468991,
  pink: 16761035,
  plum: 14524637,
  powderblue: 11591910,
  purple: 8388736,
  rebeccapurple: 6697881,
  red: 16711680,
  rosybrown: 12357519,
  royalblue: 4286945,
  saddlebrown: 9127187,
  salmon: 16416882,
  sandybrown: 16032864,
  seagreen: 3050327,
  seashell: 16774638,
  sienna: 10506797,
  silver: 12632256,
  skyblue: 8900331,
  slateblue: 6970061,
  slategray: 7372944,
  slategrey: 7372944,
  snow: 16775930,
  springgreen: 65407,
  steelblue: 4620980,
  tan: 13808780,
  teal: 32896,
  thistle: 14204888,
  tomato: 16737095,
  turquoise: 4251856,
  violet: 15631086,
  wheat: 16113331,
  white: 16777215,
  whitesmoke: 16119285,
  yellow: 16776960,
  yellowgreen: 10145074
};
define_default(Color, color, {
  copy(channels) {
    return Object.assign(new this.constructor(), this, channels);
  },
  displayable() {
    return this.rgb().displayable();
  },
  hex: color_formatHex,
  // Deprecated! Use color.formatHex.
  formatHex: color_formatHex,
  formatHex8: color_formatHex8,
  formatHsl: color_formatHsl,
  formatRgb: color_formatRgb,
  toString: color_formatRgb
});
function color_formatHex() {
  return this.rgb().formatHex();
}
function color_formatHex8() {
  return this.rgb().formatHex8();
}
function color_formatHsl() {
  return hslConvert(this).formatHsl();
}
function color_formatRgb() {
  return this.rgb().formatRgb();
}
function color(format2) {
  var m, l;
  format2 = (format2 + "").trim().toLowerCase();
  return (m = reHex.exec(format2)) ? (l = m[1].length, m = parseInt(m[1], 16), l === 6 ? rgbn(m) : l === 3 ? new Rgb(m >> 8 & 15 | m >> 4 & 240, m >> 4 & 15 | m & 240, (m & 15) << 4 | m & 15, 1) : l === 8 ? rgba(m >> 24 & 255, m >> 16 & 255, m >> 8 & 255, (m & 255) / 255) : l === 4 ? rgba(m >> 12 & 15 | m >> 8 & 240, m >> 8 & 15 | m >> 4 & 240, m >> 4 & 15 | m & 240, ((m & 15) << 4 | m & 15) / 255) : null) : (m = reRgbInteger.exec(format2)) ? new Rgb(m[1], m[2], m[3], 1) : (m = reRgbPercent.exec(format2)) ? new Rgb(m[1] * 255 / 100, m[2] * 255 / 100, m[3] * 255 / 100, 1) : (m = reRgbaInteger.exec(format2)) ? rgba(m[1], m[2], m[3], m[4]) : (m = reRgbaPercent.exec(format2)) ? rgba(m[1] * 255 / 100, m[2] * 255 / 100, m[3] * 255 / 100, m[4]) : (m = reHslPercent.exec(format2)) ? hsla(m[1], m[2] / 100, m[3] / 100, 1) : (m = reHslaPercent.exec(format2)) ? hsla(m[1], m[2] / 100, m[3] / 100, m[4]) : named.hasOwnProperty(format2) ? rgbn(named[format2]) : format2 === "transparent" ? new Rgb(NaN, NaN, NaN, 0) : null;
}
function rgbn(n) {
  return new Rgb(n >> 16 & 255, n >> 8 & 255, n & 255, 1);
}
function rgba(r, g, b, a) {
  if (a <= 0) r = g = b = NaN;
  return new Rgb(r, g, b, a);
}
function rgbConvert(o) {
  if (!(o instanceof Color)) o = color(o);
  if (!o) return new Rgb();
  o = o.rgb();
  return new Rgb(o.r, o.g, o.b, o.opacity);
}
function rgb(r, g, b, opacity) {
  return arguments.length === 1 ? rgbConvert(r) : new Rgb(r, g, b, opacity == null ? 1 : opacity);
}
function Rgb(r, g, b, opacity) {
  this.r = +r;
  this.g = +g;
  this.b = +b;
  this.opacity = +opacity;
}
define_default(Rgb, rgb, extend(Color, {
  brighter(k) {
    k = k == null ? brighter : Math.pow(brighter, k);
    return new Rgb(this.r * k, this.g * k, this.b * k, this.opacity);
  },
  darker(k) {
    k = k == null ? darker : Math.pow(darker, k);
    return new Rgb(this.r * k, this.g * k, this.b * k, this.opacity);
  },
  rgb() {
    return this;
  },
  clamp() {
    return new Rgb(clampi(this.r), clampi(this.g), clampi(this.b), clampa(this.opacity));
  },
  displayable() {
    return -0.5 <= this.r && this.r < 255.5 && (-0.5 <= this.g && this.g < 255.5) && (-0.5 <= this.b && this.b < 255.5) && (0 <= this.opacity && this.opacity <= 1);
  },
  hex: rgb_formatHex,
  // Deprecated! Use color.formatHex.
  formatHex: rgb_formatHex,
  formatHex8: rgb_formatHex8,
  formatRgb: rgb_formatRgb,
  toString: rgb_formatRgb
}));
function rgb_formatHex() {
  return `#${hex(this.r)}${hex(this.g)}${hex(this.b)}`;
}
function rgb_formatHex8() {
  return `#${hex(this.r)}${hex(this.g)}${hex(this.b)}${hex((isNaN(this.opacity) ? 1 : this.opacity) * 255)}`;
}
function rgb_formatRgb() {
  const a = clampa(this.opacity);
  return `${a === 1 ? "rgb(" : "rgba("}${clampi(this.r)}, ${clampi(this.g)}, ${clampi(this.b)}${a === 1 ? ")" : `, ${a})`}`;
}
function clampa(opacity) {
  return isNaN(opacity) ? 1 : Math.max(0, Math.min(1, opacity));
}
function clampi(value) {
  return Math.max(0, Math.min(255, Math.round(value) || 0));
}
function hex(value) {
  value = clampi(value);
  return (value < 16 ? "0" : "") + value.toString(16);
}
function hsla(h, s, l, a) {
  if (a <= 0) h = s = l = NaN;
  else if (l <= 0 || l >= 1) h = s = NaN;
  else if (s <= 0) h = NaN;
  return new Hsl(h, s, l, a);
}
function hslConvert(o) {
  if (o instanceof Hsl) return new Hsl(o.h, o.s, o.l, o.opacity);
  if (!(o instanceof Color)) o = color(o);
  if (!o) return new Hsl();
  if (o instanceof Hsl) return o;
  o = o.rgb();
  var r = o.r / 255, g = o.g / 255, b = o.b / 255, min2 = Math.min(r, g, b), max2 = Math.max(r, g, b), h = NaN, s = max2 - min2, l = (max2 + min2) / 2;
  if (s) {
    if (r === max2) h = (g - b) / s + (g < b) * 6;
    else if (g === max2) h = (b - r) / s + 2;
    else h = (r - g) / s + 4;
    s /= l < 0.5 ? max2 + min2 : 2 - max2 - min2;
    h *= 60;
  } else {
    s = l > 0 && l < 1 ? 0 : h;
  }
  return new Hsl(h, s, l, o.opacity);
}
function hsl(h, s, l, opacity) {
  return arguments.length === 1 ? hslConvert(h) : new Hsl(h, s, l, opacity == null ? 1 : opacity);
}
function Hsl(h, s, l, opacity) {
  this.h = +h;
  this.s = +s;
  this.l = +l;
  this.opacity = +opacity;
}
define_default(Hsl, hsl, extend(Color, {
  brighter(k) {
    k = k == null ? brighter : Math.pow(brighter, k);
    return new Hsl(this.h, this.s, this.l * k, this.opacity);
  },
  darker(k) {
    k = k == null ? darker : Math.pow(darker, k);
    return new Hsl(this.h, this.s, this.l * k, this.opacity);
  },
  rgb() {
    var h = this.h % 360 + (this.h < 0) * 360, s = isNaN(h) || isNaN(this.s) ? 0 : this.s, l = this.l, m2 = l + (l < 0.5 ? l : 1 - l) * s, m1 = 2 * l - m2;
    return new Rgb(
      hsl2rgb(h >= 240 ? h - 240 : h + 120, m1, m2),
      hsl2rgb(h, m1, m2),
      hsl2rgb(h < 120 ? h + 240 : h - 120, m1, m2),
      this.opacity
    );
  },
  clamp() {
    return new Hsl(clamph(this.h), clampt(this.s), clampt(this.l), clampa(this.opacity));
  },
  displayable() {
    return (0 <= this.s && this.s <= 1 || isNaN(this.s)) && (0 <= this.l && this.l <= 1) && (0 <= this.opacity && this.opacity <= 1);
  },
  formatHsl() {
    const a = clampa(this.opacity);
    return `${a === 1 ? "hsl(" : "hsla("}${clamph(this.h)}, ${clampt(this.s) * 100}%, ${clampt(this.l) * 100}%${a === 1 ? ")" : `, ${a})`}`;
  }
}));
function clamph(value) {
  value = (value || 0) % 360;
  return value < 0 ? value + 360 : value;
}
function clampt(value) {
  return Math.max(0, Math.min(1, value || 0));
}
function hsl2rgb(h, m1, m2) {
  return (h < 60 ? m1 + (m2 - m1) * h / 60 : h < 180 ? m2 : h < 240 ? m1 + (m2 - m1) * (240 - h) / 60 : m1) * 255;
}

// node_modules/d3-interpolate/src/basis.js
function basis(t12, v0, v1, v2, v3) {
  var t2 = t12 * t12, t3 = t2 * t12;
  return ((1 - 3 * t12 + 3 * t2 - t3) * v0 + (4 - 6 * t2 + 3 * t3) * v1 + (1 + 3 * t12 + 3 * t2 - 3 * t3) * v2 + t3 * v3) / 6;
}
function basis_default(values) {
  var n = values.length - 1;
  return function(t) {
    var i = t <= 0 ? t = 0 : t >= 1 ? (t = 1, n - 1) : Math.floor(t * n), v1 = values[i], v2 = values[i + 1], v0 = i > 0 ? values[i - 1] : 2 * v1 - v2, v3 = i < n - 1 ? values[i + 2] : 2 * v2 - v1;
    return basis((t - i / n) * n, v0, v1, v2, v3);
  };
}

// node_modules/d3-interpolate/src/basisClosed.js
function basisClosed_default(values) {
  var n = values.length;
  return function(t) {
    var i = Math.floor(((t %= 1) < 0 ? ++t : t) * n), v0 = values[(i + n - 1) % n], v1 = values[i % n], v2 = values[(i + 1) % n], v3 = values[(i + 2) % n];
    return basis((t - i / n) * n, v0, v1, v2, v3);
  };
}

// node_modules/d3-interpolate/src/constant.js
var constant_default2 = (x2) => () => x2;

// node_modules/d3-interpolate/src/color.js
function linear(a, d) {
  return function(t) {
    return a + t * d;
  };
}
function exponential(a, b, y2) {
  return a = Math.pow(a, y2), b = Math.pow(b, y2) - a, y2 = 1 / y2, function(t) {
    return Math.pow(a + t * b, y2);
  };
}
function gamma(y2) {
  return (y2 = +y2) === 1 ? nogamma : function(a, b) {
    return b - a ? exponential(a, b, y2) : constant_default2(isNaN(a) ? b : a);
  };
}
function nogamma(a, b) {
  var d = b - a;
  return d ? linear(a, d) : constant_default2(isNaN(a) ? b : a);
}

// node_modules/d3-interpolate/src/rgb.js
var rgb_default = function rgbGamma(y2) {
  var color2 = gamma(y2);
  function rgb2(start2, end) {
    var r = color2((start2 = rgb(start2)).r, (end = rgb(end)).r), g = color2(start2.g, end.g), b = color2(start2.b, end.b), opacity = nogamma(start2.opacity, end.opacity);
    return function(t) {
      start2.r = r(t);
      start2.g = g(t);
      start2.b = b(t);
      start2.opacity = opacity(t);
      return start2 + "";
    };
  }
  rgb2.gamma = rgbGamma;
  return rgb2;
}(1);
function rgbSpline(spline) {
  return function(colors) {
    var n = colors.length, r = new Array(n), g = new Array(n), b = new Array(n), i, color2;
    for (i = 0; i < n; ++i) {
      color2 = rgb(colors[i]);
      r[i] = color2.r || 0;
      g[i] = color2.g || 0;
      b[i] = color2.b || 0;
    }
    r = spline(r);
    g = spline(g);
    b = spline(b);
    color2.opacity = 1;
    return function(t) {
      color2.r = r(t);
      color2.g = g(t);
      color2.b = b(t);
      return color2 + "";
    };
  };
}
var rgbBasis = rgbSpline(basis_default);
var rgbBasisClosed = rgbSpline(basisClosed_default);

// node_modules/d3-interpolate/src/numberArray.js
function numberArray_default(a, b) {
  if (!b) b = [];
  var n = a ? Math.min(b.length, a.length) : 0, c = b.slice(), i;
  return function(t) {
    for (i = 0; i < n; ++i) c[i] = a[i] * (1 - t) + b[i] * t;
    return c;
  };
}
function isNumberArray(x2) {
  return ArrayBuffer.isView(x2) && !(x2 instanceof DataView);
}

// node_modules/d3-interpolate/src/array.js
function genericArray(a, b) {
  var nb = b ? b.length : 0, na = a ? Math.min(nb, a.length) : 0, x2 = new Array(na), c = new Array(nb), i;
  for (i = 0; i < na; ++i) x2[i] = value_default(a[i], b[i]);
  for (; i < nb; ++i) c[i] = b[i];
  return function(t) {
    for (i = 0; i < na; ++i) c[i] = x2[i](t);
    return c;
  };
}

// node_modules/d3-interpolate/src/date.js
function date_default(a, b) {
  var d = /* @__PURE__ */ new Date();
  return a = +a, b = +b, function(t) {
    return d.setTime(a * (1 - t) + b * t), d;
  };
}

// node_modules/d3-interpolate/src/number.js
function number_default(a, b) {
  return a = +a, b = +b, function(t) {
    return a * (1 - t) + b * t;
  };
}

// node_modules/d3-interpolate/src/object.js
function object_default(a, b) {
  var i = {}, c = {}, k;
  if (a === null || typeof a !== "object") a = {};
  if (b === null || typeof b !== "object") b = {};
  for (k in b) {
    if (k in a) {
      i[k] = value_default(a[k], b[k]);
    } else {
      c[k] = b[k];
    }
  }
  return function(t) {
    for (k in i) c[k] = i[k](t);
    return c;
  };
}

// node_modules/d3-interpolate/src/string.js
var reA = /[-+]?(?:\d+\.?\d*|\.?\d+)(?:[eE][-+]?\d+)?/g;
var reB = new RegExp(reA.source, "g");
function zero2(b) {
  return function() {
    return b;
  };
}
function one(b) {
  return function(t) {
    return b(t) + "";
  };
}
function string_default(a, b) {
  var bi = reA.lastIndex = reB.lastIndex = 0, am, bm, bs, i = -1, s = [], q = [];
  a = a + "", b = b + "";
  while ((am = reA.exec(a)) && (bm = reB.exec(b))) {
    if ((bs = bm.index) > bi) {
      bs = b.slice(bi, bs);
      if (s[i]) s[i] += bs;
      else s[++i] = bs;
    }
    if ((am = am[0]) === (bm = bm[0])) {
      if (s[i]) s[i] += bm;
      else s[++i] = bm;
    } else {
      s[++i] = null;
      q.push({ i, x: number_default(am, bm) });
    }
    bi = reB.lastIndex;
  }
  if (bi < b.length) {
    bs = b.slice(bi);
    if (s[i]) s[i] += bs;
    else s[++i] = bs;
  }
  return s.length < 2 ? q[0] ? one(q[0].x) : zero2(b) : (b = q.length, function(t) {
    for (var i2 = 0, o; i2 < b; ++i2) s[(o = q[i2]).i] = o.x(t);
    return s.join("");
  });
}

// node_modules/d3-interpolate/src/value.js
function value_default(a, b) {
  var t = typeof b, c;
  return b == null || t === "boolean" ? constant_default2(b) : (t === "number" ? number_default : t === "string" ? (c = color(b)) ? (b = c, rgb_default) : string_default : b instanceof color ? rgb_default : b instanceof Date ? date_default : isNumberArray(b) ? numberArray_default : Array.isArray(b) ? genericArray : typeof b.valueOf !== "function" && typeof b.toString !== "function" || isNaN(b) ? object_default : number_default)(a, b);
}

// node_modules/d3-interpolate/src/round.js
function round_default(a, b) {
  return a = +a, b = +b, function(t) {
    return Math.round(a * (1 - t) + b * t);
  };
}

// node_modules/d3-interpolate/src/transform/decompose.js
var degrees = 180 / Math.PI;
var identity = {
  translateX: 0,
  translateY: 0,
  rotate: 0,
  skewX: 0,
  scaleX: 1,
  scaleY: 1
};
function decompose_default(a, b, c, d, e, f) {
  var scaleX, scaleY, skewX;
  if (scaleX = Math.sqrt(a * a + b * b)) a /= scaleX, b /= scaleX;
  if (skewX = a * c + b * d) c -= a * skewX, d -= b * skewX;
  if (scaleY = Math.sqrt(c * c + d * d)) c /= scaleY, d /= scaleY, skewX /= scaleY;
  if (a * d < b * c) a = -a, b = -b, skewX = -skewX, scaleX = -scaleX;
  return {
    translateX: e,
    translateY: f,
    rotate: Math.atan2(b, a) * degrees,
    skewX: Math.atan(skewX) * degrees,
    scaleX,
    scaleY
  };
}

// node_modules/d3-interpolate/src/transform/parse.js
var svgNode;
function parseCss(value) {
  const m = new (typeof DOMMatrix === "function" ? DOMMatrix : WebKitCSSMatrix)(value + "");
  return m.isIdentity ? identity : decompose_default(m.a, m.b, m.c, m.d, m.e, m.f);
}
function parseSvg(value) {
  if (value == null) return identity;
  if (!svgNode) svgNode = document.createElementNS("http://www.w3.org/2000/svg", "g");
  svgNode.setAttribute("transform", value);
  if (!(value = svgNode.transform.baseVal.consolidate())) return identity;
  value = value.matrix;
  return decompose_default(value.a, value.b, value.c, value.d, value.e, value.f);
}

// node_modules/d3-interpolate/src/transform/index.js
function interpolateTransform(parse, pxComma, pxParen, degParen) {
  function pop(s) {
    return s.length ? s.pop() + " " : "";
  }
  function translate(xa, ya, xb, yb, s, q) {
    if (xa !== xb || ya !== yb) {
      var i = s.push("translate(", null, pxComma, null, pxParen);
      q.push({ i: i - 4, x: number_default(xa, xb) }, { i: i - 2, x: number_default(ya, yb) });
    } else if (xb || yb) {
      s.push("translate(" + xb + pxComma + yb + pxParen);
    }
  }
  function rotate(a, b, s, q) {
    if (a !== b) {
      if (a - b > 180) b += 360;
      else if (b - a > 180) a += 360;
      q.push({ i: s.push(pop(s) + "rotate(", null, degParen) - 2, x: number_default(a, b) });
    } else if (b) {
      s.push(pop(s) + "rotate(" + b + degParen);
    }
  }
  function skewX(a, b, s, q) {
    if (a !== b) {
      q.push({ i: s.push(pop(s) + "skewX(", null, degParen) - 2, x: number_default(a, b) });
    } else if (b) {
      s.push(pop(s) + "skewX(" + b + degParen);
    }
  }
  function scale(xa, ya, xb, yb, s, q) {
    if (xa !== xb || ya !== yb) {
      var i = s.push(pop(s) + "scale(", null, ",", null, ")");
      q.push({ i: i - 4, x: number_default(xa, xb) }, { i: i - 2, x: number_default(ya, yb) });
    } else if (xb !== 1 || yb !== 1) {
      s.push(pop(s) + "scale(" + xb + "," + yb + ")");
    }
  }
  return function(a, b) {
    var s = [], q = [];
    a = parse(a), b = parse(b);
    translate(a.translateX, a.translateY, b.translateX, b.translateY, s, q);
    rotate(a.rotate, b.rotate, s, q);
    skewX(a.skewX, b.skewX, s, q);
    scale(a.scaleX, a.scaleY, b.scaleX, b.scaleY, s, q);
    a = b = null;
    return function(t) {
      var i = -1, n = q.length, o;
      while (++i < n) s[(o = q[i]).i] = o.x(t);
      return s.join("");
    };
  };
}
var interpolateTransformCss = interpolateTransform(parseCss, "px, ", "px)", "deg)");
var interpolateTransformSvg = interpolateTransform(parseSvg, ", ", ")", ")");

// node_modules/d3-interpolate/src/zoom.js
var epsilon2 = 1e-12;
function cosh(x2) {
  return ((x2 = Math.exp(x2)) + 1 / x2) / 2;
}
function sinh(x2) {
  return ((x2 = Math.exp(x2)) - 1 / x2) / 2;
}
function tanh(x2) {
  return ((x2 = Math.exp(2 * x2)) - 1) / (x2 + 1);
}
var zoom_default = function zoomRho(rho, rho2, rho4) {
  function zoom(p0, p1) {
    var ux0 = p0[0], uy0 = p0[1], w0 = p0[2], ux1 = p1[0], uy1 = p1[1], w1 = p1[2], dx = ux1 - ux0, dy = uy1 - uy0, d2 = dx * dx + dy * dy, i, S;
    if (d2 < epsilon2) {
      S = Math.log(w1 / w0) / rho;
      i = function(t) {
        return [
          ux0 + t * dx,
          uy0 + t * dy,
          w0 * Math.exp(rho * t * S)
        ];
      };
    } else {
      var d1 = Math.sqrt(d2), b0 = (w1 * w1 - w0 * w0 + rho4 * d2) / (2 * w0 * rho2 * d1), b1 = (w1 * w1 - w0 * w0 - rho4 * d2) / (2 * w1 * rho2 * d1), r0 = Math.log(Math.sqrt(b0 * b0 + 1) - b0), r1 = Math.log(Math.sqrt(b1 * b1 + 1) - b1);
      S = (r1 - r0) / rho;
      i = function(t) {
        var s = t * S, coshr0 = cosh(r0), u = w0 / (rho2 * d1) * (coshr0 * tanh(rho * s + r0) - sinh(r0));
        return [
          ux0 + u * dx,
          uy0 + u * dy,
          w0 * coshr0 / cosh(rho * s + r0)
        ];
      };
    }
    i.duration = S * 1e3 * rho / Math.SQRT2;
    return i;
  }
  zoom.rho = function(_) {
    var _1 = Math.max(1e-3, +_), _2 = _1 * _1, _4 = _2 * _2;
    return zoomRho(_1, _2, _4);
  };
  return zoom;
}(Math.SQRT2, 2, 4);

// node_modules/d3-timer/src/timer.js
var frame = 0;
var timeout = 0;
var interval = 0;
var pokeDelay = 1e3;
var taskHead;
var taskTail;
var clockLast = 0;
var clockNow = 0;
var clockSkew = 0;
var clock = typeof performance === "object" && performance.now ? performance : Date;
var setFrame = typeof window === "object" && window.requestAnimationFrame ? window.requestAnimationFrame.bind(window) : function(f) {
  setTimeout(f, 17);
};
function now() {
  return clockNow || (setFrame(clearNow), clockNow = clock.now() + clockSkew);
}
function clearNow() {
  clockNow = 0;
}
function Timer() {
  this._call = this._time = this._next = null;
}
Timer.prototype = timer.prototype = {
  constructor: Timer,
  restart: function(callback, delay, time) {
    if (typeof callback !== "function") throw new TypeError("callback is not a function");
    time = (time == null ? now() : +time) + (delay == null ? 0 : +delay);
    if (!this._next && taskTail !== this) {
      if (taskTail) taskTail._next = this;
      else taskHead = this;
      taskTail = this;
    }
    this._call = callback;
    this._time = time;
    sleep();
  },
  stop: function() {
    if (this._call) {
      this._call = null;
      this._time = Infinity;
      sleep();
    }
  }
};
function timer(callback, delay, time) {
  var t = new Timer();
  t.restart(callback, delay, time);
  return t;
}
function timerFlush() {
  now();
  ++frame;
  var t = taskHead, e;
  while (t) {
    if ((e = clockNow - t._time) >= 0) t._call.call(void 0, e);
    t = t._next;
  }
  --frame;
}
function wake() {
  clockNow = (clockLast = clock.now()) + clockSkew;
  frame = timeout = 0;
  try {
    timerFlush();
  } finally {
    frame = 0;
    nap();
    clockNow = 0;
  }
}
function poke() {
  var now2 = clock.now(), delay = now2 - clockLast;
  if (delay > pokeDelay) clockSkew -= delay, clockLast = now2;
}
function nap() {
  var t02, t12 = taskHead, t2, time = Infinity;
  while (t12) {
    if (t12._call) {
      if (time > t12._time) time = t12._time;
      t02 = t12, t12 = t12._next;
    } else {
      t2 = t12._next, t12._next = null;
      t12 = t02 ? t02._next = t2 : taskHead = t2;
    }
  }
  taskTail = t02;
  sleep(time);
}
function sleep(time) {
  if (frame) return;
  if (timeout) timeout = clearTimeout(timeout);
  var delay = time - clockNow;
  if (delay > 24) {
    if (time < Infinity) timeout = setTimeout(wake, time - clock.now() - clockSkew);
    if (interval) interval = clearInterval(interval);
  } else {
    if (!interval) clockLast = clock.now(), interval = setInterval(poke, pokeDelay);
    frame = 1, setFrame(wake);
  }
}

// node_modules/d3-timer/src/timeout.js
function timeout_default(callback, delay, time) {
  var t = new Timer();
  delay = delay == null ? 0 : +delay;
  t.restart((elapsed) => {
    t.stop();
    callback(elapsed + delay);
  }, delay, time);
  return t;
}

// node_modules/d3-transition/src/transition/schedule.js
var emptyOn = dispatch_default("start", "end", "cancel", "interrupt");
var emptyTween = [];
var CREATED = 0;
var SCHEDULED = 1;
var STARTING = 2;
var STARTED = 3;
var RUNNING = 4;
var ENDING = 5;
var ENDED = 6;
function schedule_default(node, name, id2, index, group, timing) {
  var schedules = node.__transition;
  if (!schedules) node.__transition = {};
  else if (id2 in schedules) return;
  create(node, id2, {
    name,
    index,
    // For context during callback.
    group,
    // For context during callback.
    on: emptyOn,
    tween: emptyTween,
    time: timing.time,
    delay: timing.delay,
    duration: timing.duration,
    ease: timing.ease,
    timer: null,
    state: CREATED
  });
}
function init(node, id2) {
  var schedule = get2(node, id2);
  if (schedule.state > CREATED) throw new Error("too late; already scheduled");
  return schedule;
}
function set2(node, id2) {
  var schedule = get2(node, id2);
  if (schedule.state > STARTED) throw new Error("too late; already running");
  return schedule;
}
function get2(node, id2) {
  var schedule = node.__transition;
  if (!schedule || !(schedule = schedule[id2])) throw new Error("transition not found");
  return schedule;
}
function create(node, id2, self) {
  var schedules = node.__transition, tween;
  schedules[id2] = self;
  self.timer = timer(schedule, 0, self.time);
  function schedule(elapsed) {
    self.state = SCHEDULED;
    self.timer.restart(start2, self.delay, self.time);
    if (self.delay <= elapsed) start2(elapsed - self.delay);
  }
  function start2(elapsed) {
    var i, j, n, o;
    if (self.state !== SCHEDULED) return stop();
    for (i in schedules) {
      o = schedules[i];
      if (o.name !== self.name) continue;
      if (o.state === STARTED) return timeout_default(start2);
      if (o.state === RUNNING) {
        o.state = ENDED;
        o.timer.stop();
        o.on.call("interrupt", node, node.__data__, o.index, o.group);
        delete schedules[i];
      } else if (+i < id2) {
        o.state = ENDED;
        o.timer.stop();
        o.on.call("cancel", node, node.__data__, o.index, o.group);
        delete schedules[i];
      }
    }
    timeout_default(function() {
      if (self.state === STARTED) {
        self.state = RUNNING;
        self.timer.restart(tick, self.delay, self.time);
        tick(elapsed);
      }
    });
    self.state = STARTING;
    self.on.call("start", node, node.__data__, self.index, self.group);
    if (self.state !== STARTING) return;
    self.state = STARTED;
    tween = new Array(n = self.tween.length);
    for (i = 0, j = -1; i < n; ++i) {
      if (o = self.tween[i].value.call(node, node.__data__, self.index, self.group)) {
        tween[++j] = o;
      }
    }
    tween.length = j + 1;
  }
  function tick(elapsed) {
    var t = elapsed < self.duration ? self.ease.call(null, elapsed / self.duration) : (self.timer.restart(stop), self.state = ENDING, 1), i = -1, n = tween.length;
    while (++i < n) {
      tween[i].call(node, t);
    }
    if (self.state === ENDING) {
      self.on.call("end", node, node.__data__, self.index, self.group);
      stop();
    }
  }
  function stop() {
    self.state = ENDED;
    self.timer.stop();
    delete schedules[id2];
    for (var i in schedules) return;
    delete node.__transition;
  }
}

// node_modules/d3-transition/src/interrupt.js
function interrupt_default(node, name) {
  var schedules = node.__transition, schedule, active, empty2 = true, i;
  if (!schedules) return;
  name = name == null ? null : name + "";
  for (i in schedules) {
    if ((schedule = schedules[i]).name !== name) {
      empty2 = false;
      continue;
    }
    active = schedule.state > STARTING && schedule.state < ENDING;
    schedule.state = ENDED;
    schedule.timer.stop();
    schedule.on.call(active ? "interrupt" : "cancel", node, node.__data__, schedule.index, schedule.group);
    delete schedules[i];
  }
  if (empty2) delete node.__transition;
}

// node_modules/d3-transition/src/selection/interrupt.js
function interrupt_default2(name) {
  return this.each(function() {
    interrupt_default(this, name);
  });
}

// node_modules/d3-transition/src/transition/tween.js
function tweenRemove(id2, name) {
  var tween0, tween1;
  return function() {
    var schedule = set2(this, id2), tween = schedule.tween;
    if (tween !== tween0) {
      tween1 = tween0 = tween;
      for (var i = 0, n = tween1.length; i < n; ++i) {
        if (tween1[i].name === name) {
          tween1 = tween1.slice();
          tween1.splice(i, 1);
          break;
        }
      }
    }
    schedule.tween = tween1;
  };
}
function tweenFunction(id2, name, value) {
  var tween0, tween1;
  if (typeof value !== "function") throw new Error();
  return function() {
    var schedule = set2(this, id2), tween = schedule.tween;
    if (tween !== tween0) {
      tween1 = (tween0 = tween).slice();
      for (var t = { name, value }, i = 0, n = tween1.length; i < n; ++i) {
        if (tween1[i].name === name) {
          tween1[i] = t;
          break;
        }
      }
      if (i === n) tween1.push(t);
    }
    schedule.tween = tween1;
  };
}
function tween_default(name, value) {
  var id2 = this._id;
  name += "";
  if (arguments.length < 2) {
    var tween = get2(this.node(), id2).tween;
    for (var i = 0, n = tween.length, t; i < n; ++i) {
      if ((t = tween[i]).name === name) {
        return t.value;
      }
    }
    return null;
  }
  return this.each((value == null ? tweenRemove : tweenFunction)(id2, name, value));
}
function tweenValue(transition2, name, value) {
  var id2 = transition2._id;
  transition2.each(function() {
    var schedule = set2(this, id2);
    (schedule.value || (schedule.value = {}))[name] = value.apply(this, arguments);
  });
  return function(node) {
    return get2(node, id2).value[name];
  };
}

// node_modules/d3-transition/src/transition/interpolate.js
function interpolate_default(a, b) {
  var c;
  return (typeof b === "number" ? number_default : b instanceof color ? rgb_default : (c = color(b)) ? (b = c, rgb_default) : string_default)(a, b);
}

// node_modules/d3-transition/src/transition/attr.js
function attrRemove2(name) {
  return function() {
    this.removeAttribute(name);
  };
}
function attrRemoveNS2(fullname) {
  return function() {
    this.removeAttributeNS(fullname.space, fullname.local);
  };
}
function attrConstant2(name, interpolate, value1) {
  var string00, string1 = value1 + "", interpolate0;
  return function() {
    var string0 = this.getAttribute(name);
    return string0 === string1 ? null : string0 === string00 ? interpolate0 : interpolate0 = interpolate(string00 = string0, value1);
  };
}
function attrConstantNS2(fullname, interpolate, value1) {
  var string00, string1 = value1 + "", interpolate0;
  return function() {
    var string0 = this.getAttributeNS(fullname.space, fullname.local);
    return string0 === string1 ? null : string0 === string00 ? interpolate0 : interpolate0 = interpolate(string00 = string0, value1);
  };
}
function attrFunction2(name, interpolate, value) {
  var string00, string10, interpolate0;
  return function() {
    var string0, value1 = value(this), string1;
    if (value1 == null) return void this.removeAttribute(name);
    string0 = this.getAttribute(name);
    string1 = value1 + "";
    return string0 === string1 ? null : string0 === string00 && string1 === string10 ? interpolate0 : (string10 = string1, interpolate0 = interpolate(string00 = string0, value1));
  };
}
function attrFunctionNS2(fullname, interpolate, value) {
  var string00, string10, interpolate0;
  return function() {
    var string0, value1 = value(this), string1;
    if (value1 == null) return void this.removeAttributeNS(fullname.space, fullname.local);
    string0 = this.getAttributeNS(fullname.space, fullname.local);
    string1 = value1 + "";
    return string0 === string1 ? null : string0 === string00 && string1 === string10 ? interpolate0 : (string10 = string1, interpolate0 = interpolate(string00 = string0, value1));
  };
}
function attr_default2(name, value) {
  var fullname = namespace_default(name), i = fullname === "transform" ? interpolateTransformSvg : interpolate_default;
  return this.attrTween(name, typeof value === "function" ? (fullname.local ? attrFunctionNS2 : attrFunction2)(fullname, i, tweenValue(this, "attr." + name, value)) : value == null ? (fullname.local ? attrRemoveNS2 : attrRemove2)(fullname) : (fullname.local ? attrConstantNS2 : attrConstant2)(fullname, i, value));
}

// node_modules/d3-transition/src/transition/attrTween.js
function attrInterpolate(name, i) {
  return function(t) {
    this.setAttribute(name, i.call(this, t));
  };
}
function attrInterpolateNS(fullname, i) {
  return function(t) {
    this.setAttributeNS(fullname.space, fullname.local, i.call(this, t));
  };
}
function attrTweenNS(fullname, value) {
  var t02, i0;
  function tween() {
    var i = value.apply(this, arguments);
    if (i !== i0) t02 = (i0 = i) && attrInterpolateNS(fullname, i);
    return t02;
  }
  tween._value = value;
  return tween;
}
function attrTween(name, value) {
  var t02, i0;
  function tween() {
    var i = value.apply(this, arguments);
    if (i !== i0) t02 = (i0 = i) && attrInterpolate(name, i);
    return t02;
  }
  tween._value = value;
  return tween;
}
function attrTween_default(name, value) {
  var key = "attr." + name;
  if (arguments.length < 2) return (key = this.tween(key)) && key._value;
  if (value == null) return this.tween(key, null);
  if (typeof value !== "function") throw new Error();
  var fullname = namespace_default(name);
  return this.tween(key, (fullname.local ? attrTweenNS : attrTween)(fullname, value));
}

// node_modules/d3-transition/src/transition/delay.js
function delayFunction(id2, value) {
  return function() {
    init(this, id2).delay = +value.apply(this, arguments);
  };
}
function delayConstant(id2, value) {
  return value = +value, function() {
    init(this, id2).delay = value;
  };
}
function delay_default(value) {
  var id2 = this._id;
  return arguments.length ? this.each((typeof value === "function" ? delayFunction : delayConstant)(id2, value)) : get2(this.node(), id2).delay;
}

// node_modules/d3-transition/src/transition/duration.js
function durationFunction(id2, value) {
  return function() {
    set2(this, id2).duration = +value.apply(this, arguments);
  };
}
function durationConstant(id2, value) {
  return value = +value, function() {
    set2(this, id2).duration = value;
  };
}
function duration_default(value) {
  var id2 = this._id;
  return arguments.length ? this.each((typeof value === "function" ? durationFunction : durationConstant)(id2, value)) : get2(this.node(), id2).duration;
}

// node_modules/d3-transition/src/transition/ease.js
function easeConstant(id2, value) {
  if (typeof value !== "function") throw new Error();
  return function() {
    set2(this, id2).ease = value;
  };
}
function ease_default(value) {
  var id2 = this._id;
  return arguments.length ? this.each(easeConstant(id2, value)) : get2(this.node(), id2).ease;
}

// node_modules/d3-transition/src/transition/easeVarying.js
function easeVarying(id2, value) {
  return function() {
    var v = value.apply(this, arguments);
    if (typeof v !== "function") throw new Error();
    set2(this, id2).ease = v;
  };
}
function easeVarying_default(value) {
  if (typeof value !== "function") throw new Error();
  return this.each(easeVarying(this._id, value));
}

// node_modules/d3-transition/src/transition/filter.js
function filter_default2(match) {
  if (typeof match !== "function") match = matcher_default(match);
  for (var groups = this._groups, m = groups.length, subgroups = new Array(m), j = 0; j < m; ++j) {
    for (var group = groups[j], n = group.length, subgroup = subgroups[j] = [], node, i = 0; i < n; ++i) {
      if ((node = group[i]) && match.call(node, node.__data__, i, group)) {
        subgroup.push(node);
      }
    }
  }
  return new Transition(subgroups, this._parents, this._name, this._id);
}

// node_modules/d3-transition/src/transition/merge.js
function merge_default2(transition2) {
  if (transition2._id !== this._id) throw new Error();
  for (var groups0 = this._groups, groups1 = transition2._groups, m0 = groups0.length, m1 = groups1.length, m = Math.min(m0, m1), merges = new Array(m0), j = 0; j < m; ++j) {
    for (var group0 = groups0[j], group1 = groups1[j], n = group0.length, merge = merges[j] = new Array(n), node, i = 0; i < n; ++i) {
      if (node = group0[i] || group1[i]) {
        merge[i] = node;
      }
    }
  }
  for (; j < m0; ++j) {
    merges[j] = groups0[j];
  }
  return new Transition(merges, this._parents, this._name, this._id);
}

// node_modules/d3-transition/src/transition/on.js
function start(name) {
  return (name + "").trim().split(/^|\s+/).every(function(t) {
    var i = t.indexOf(".");
    if (i >= 0) t = t.slice(0, i);
    return !t || t === "start";
  });
}
function onFunction(id2, name, listener) {
  var on0, on1, sit = start(name) ? init : set2;
  return function() {
    var schedule = sit(this, id2), on = schedule.on;
    if (on !== on0) (on1 = (on0 = on).copy()).on(name, listener);
    schedule.on = on1;
  };
}
function on_default2(name, listener) {
  var id2 = this._id;
  return arguments.length < 2 ? get2(this.node(), id2).on.on(name) : this.each(onFunction(id2, name, listener));
}

// node_modules/d3-transition/src/transition/remove.js
function removeFunction(id2) {
  return function() {
    var parent = this.parentNode;
    for (var i in this.__transition) if (+i !== id2) return;
    if (parent) parent.removeChild(this);
  };
}
function remove_default2() {
  return this.on("end.remove", removeFunction(this._id));
}

// node_modules/d3-transition/src/transition/select.js
function select_default3(select) {
  var name = this._name, id2 = this._id;
  if (typeof select !== "function") select = selector_default(select);
  for (var groups = this._groups, m = groups.length, subgroups = new Array(m), j = 0; j < m; ++j) {
    for (var group = groups[j], n = group.length, subgroup = subgroups[j] = new Array(n), node, subnode, i = 0; i < n; ++i) {
      if ((node = group[i]) && (subnode = select.call(node, node.__data__, i, group))) {
        if ("__data__" in node) subnode.__data__ = node.__data__;
        subgroup[i] = subnode;
        schedule_default(subgroup[i], name, id2, i, subgroup, get2(node, id2));
      }
    }
  }
  return new Transition(subgroups, this._parents, name, id2);
}

// node_modules/d3-transition/src/transition/selectAll.js
function selectAll_default2(select) {
  var name = this._name, id2 = this._id;
  if (typeof select !== "function") select = selectorAll_default(select);
  for (var groups = this._groups, m = groups.length, subgroups = [], parents = [], j = 0; j < m; ++j) {
    for (var group = groups[j], n = group.length, node, i = 0; i < n; ++i) {
      if (node = group[i]) {
        for (var children2 = select.call(node, node.__data__, i, group), child, inherit2 = get2(node, id2), k = 0, l = children2.length; k < l; ++k) {
          if (child = children2[k]) {
            schedule_default(child, name, id2, k, children2, inherit2);
          }
        }
        subgroups.push(children2);
        parents.push(node);
      }
    }
  }
  return new Transition(subgroups, parents, name, id2);
}

// node_modules/d3-transition/src/transition/selection.js
var Selection2 = selection_default.prototype.constructor;
function selection_default2() {
  return new Selection2(this._groups, this._parents);
}

// node_modules/d3-transition/src/transition/style.js
function styleNull(name, interpolate) {
  var string00, string10, interpolate0;
  return function() {
    var string0 = styleValue(this, name), string1 = (this.style.removeProperty(name), styleValue(this, name));
    return string0 === string1 ? null : string0 === string00 && string1 === string10 ? interpolate0 : interpolate0 = interpolate(string00 = string0, string10 = string1);
  };
}
function styleRemove2(name) {
  return function() {
    this.style.removeProperty(name);
  };
}
function styleConstant2(name, interpolate, value1) {
  var string00, string1 = value1 + "", interpolate0;
  return function() {
    var string0 = styleValue(this, name);
    return string0 === string1 ? null : string0 === string00 ? interpolate0 : interpolate0 = interpolate(string00 = string0, value1);
  };
}
function styleFunction2(name, interpolate, value) {
  var string00, string10, interpolate0;
  return function() {
    var string0 = styleValue(this, name), value1 = value(this), string1 = value1 + "";
    if (value1 == null) string1 = value1 = (this.style.removeProperty(name), styleValue(this, name));
    return string0 === string1 ? null : string0 === string00 && string1 === string10 ? interpolate0 : (string10 = string1, interpolate0 = interpolate(string00 = string0, value1));
  };
}
function styleMaybeRemove(id2, name) {
  var on0, on1, listener0, key = "style." + name, event = "end." + key, remove2;
  return function() {
    var schedule = set2(this, id2), on = schedule.on, listener = schedule.value[key] == null ? remove2 || (remove2 = styleRemove2(name)) : void 0;
    if (on !== on0 || listener0 !== listener) (on1 = (on0 = on).copy()).on(event, listener0 = listener);
    schedule.on = on1;
  };
}
function style_default2(name, value, priority) {
  var i = (name += "") === "transform" ? interpolateTransformCss : interpolate_default;
  return value == null ? this.styleTween(name, styleNull(name, i)).on("end.style." + name, styleRemove2(name)) : typeof value === "function" ? this.styleTween(name, styleFunction2(name, i, tweenValue(this, "style." + name, value))).each(styleMaybeRemove(this._id, name)) : this.styleTween(name, styleConstant2(name, i, value), priority).on("end.style." + name, null);
}

// node_modules/d3-transition/src/transition/styleTween.js
function styleInterpolate(name, i, priority) {
  return function(t) {
    this.style.setProperty(name, i.call(this, t), priority);
  };
}
function styleTween(name, value, priority) {
  var t, i0;
  function tween() {
    var i = value.apply(this, arguments);
    if (i !== i0) t = (i0 = i) && styleInterpolate(name, i, priority);
    return t;
  }
  tween._value = value;
  return tween;
}
function styleTween_default(name, value, priority) {
  var key = "style." + (name += "");
  if (arguments.length < 2) return (key = this.tween(key)) && key._value;
  if (value == null) return this.tween(key, null);
  if (typeof value !== "function") throw new Error();
  return this.tween(key, styleTween(name, value, priority == null ? "" : priority));
}

// node_modules/d3-transition/src/transition/text.js
function textConstant2(value) {
  return function() {
    this.textContent = value;
  };
}
function textFunction2(value) {
  return function() {
    var value1 = value(this);
    this.textContent = value1 == null ? "" : value1;
  };
}
function text_default2(value) {
  return this.tween("text", typeof value === "function" ? textFunction2(tweenValue(this, "text", value)) : textConstant2(value == null ? "" : value + ""));
}

// node_modules/d3-transition/src/transition/textTween.js
function textInterpolate(i) {
  return function(t) {
    this.textContent = i.call(this, t);
  };
}
function textTween(value) {
  var t02, i0;
  function tween() {
    var i = value.apply(this, arguments);
    if (i !== i0) t02 = (i0 = i) && textInterpolate(i);
    return t02;
  }
  tween._value = value;
  return tween;
}
function textTween_default(value) {
  var key = "text";
  if (arguments.length < 1) return (key = this.tween(key)) && key._value;
  if (value == null) return this.tween(key, null);
  if (typeof value !== "function") throw new Error();
  return this.tween(key, textTween(value));
}

// node_modules/d3-transition/src/transition/transition.js
function transition_default() {
  var name = this._name, id0 = this._id, id1 = newId();
  for (var groups = this._groups, m = groups.length, j = 0; j < m; ++j) {
    for (var group = groups[j], n = group.length, node, i = 0; i < n; ++i) {
      if (node = group[i]) {
        var inherit2 = get2(node, id0);
        schedule_default(node, name, id1, i, group, {
          time: inherit2.time + inherit2.delay + inherit2.duration,
          delay: 0,
          duration: inherit2.duration,
          ease: inherit2.ease
        });
      }
    }
  }
  return new Transition(groups, this._parents, name, id1);
}

// node_modules/d3-transition/src/transition/end.js
function end_default() {
  var on0, on1, that = this, id2 = that._id, size = that.size();
  return new Promise(function(resolve, reject) {
    var cancel = { value: reject }, end = { value: function() {
      if (--size === 0) resolve();
    } };
    that.each(function() {
      var schedule = set2(this, id2), on = schedule.on;
      if (on !== on0) {
        on1 = (on0 = on).copy();
        on1._.cancel.push(cancel);
        on1._.interrupt.push(cancel);
        on1._.end.push(end);
      }
      schedule.on = on1;
    });
    if (size === 0) resolve();
  });
}

// node_modules/d3-transition/src/transition/index.js
var id = 0;
function Transition(groups, parents, name, id2) {
  this._groups = groups;
  this._parents = parents;
  this._name = name;
  this._id = id2;
}
function transition(name) {
  return selection_default().transition(name);
}
function newId() {
  return ++id;
}
var selection_prototype = selection_default.prototype;
Transition.prototype = transition.prototype = {
  constructor: Transition,
  select: select_default3,
  selectAll: selectAll_default2,
  selectChild: selection_prototype.selectChild,
  selectChildren: selection_prototype.selectChildren,
  filter: filter_default2,
  merge: merge_default2,
  selection: selection_default2,
  transition: transition_default,
  call: selection_prototype.call,
  nodes: selection_prototype.nodes,
  node: selection_prototype.node,
  size: selection_prototype.size,
  empty: selection_prototype.empty,
  each: selection_prototype.each,
  on: on_default2,
  attr: attr_default2,
  attrTween: attrTween_default,
  style: style_default2,
  styleTween: styleTween_default,
  text: text_default2,
  textTween: textTween_default,
  remove: remove_default2,
  tween: tween_default,
  delay: delay_default,
  duration: duration_default,
  ease: ease_default,
  easeVarying: easeVarying_default,
  end: end_default,
  [Symbol.iterator]: selection_prototype[Symbol.iterator]
};

// node_modules/d3-ease/src/cubic.js
function cubicInOut(t) {
  return ((t *= 2) <= 1 ? t * t * t : (t -= 2) * t * t + 2) / 2;
}

// node_modules/d3-transition/src/selection/transition.js
var defaultTiming = {
  time: null,
  // Set on use.
  delay: 0,
  duration: 250,
  ease: cubicInOut
};
function inherit(node, id2) {
  var timing;
  while (!(timing = node.__transition) || !(timing = timing[id2])) {
    if (!(node = node.parentNode)) {
      throw new Error(`transition ${id2} not found`);
    }
  }
  return timing;
}
function transition_default2(name) {
  var id2, timing;
  if (name instanceof Transition) {
    id2 = name._id, name = name._name;
  } else {
    id2 = newId(), (timing = defaultTiming).time = now(), name = name == null ? null : name + "";
  }
  for (var groups = this._groups, m = groups.length, j = 0; j < m; ++j) {
    for (var group = groups[j], n = group.length, node, i = 0; i < n; ++i) {
      if (node = group[i]) {
        schedule_default(node, name, id2, i, group, timing || inherit(node, id2));
      }
    }
  }
  return new Transition(groups, this._parents, name, id2);
}

// node_modules/d3-transition/src/selection/index.js
selection_default.prototype.interrupt = interrupt_default2;
selection_default.prototype.transition = transition_default2;

// node_modules/d3-brush/src/brush.js
var { abs, max, min } = Math;
function number1(e) {
  return [+e[0], +e[1]];
}
function number22(e) {
  return [number1(e[0]), number1(e[1])];
}
var X = {
  name: "x",
  handles: ["w", "e"].map(type),
  input: function(x2, e) {
    return x2 == null ? null : [[+x2[0], e[0][1]], [+x2[1], e[1][1]]];
  },
  output: function(xy) {
    return xy && [xy[0][0], xy[1][0]];
  }
};
var Y = {
  name: "y",
  handles: ["n", "s"].map(type),
  input: function(y2, e) {
    return y2 == null ? null : [[e[0][0], +y2[0]], [e[1][0], +y2[1]]];
  },
  output: function(xy) {
    return xy && [xy[0][1], xy[1][1]];
  }
};
var XY = {
  name: "xy",
  handles: ["n", "w", "e", "s", "nw", "ne", "sw", "se"].map(type),
  input: function(xy) {
    return xy == null ? null : number22(xy);
  },
  output: function(xy) {
    return xy;
  }
};
function type(t) {
  return { type: t };
}

// node_modules/d3-path/src/path.js
var pi = Math.PI;
var tau = 2 * pi;
var epsilon3 = 1e-6;
var tauEpsilon = tau - epsilon3;
function append(strings) {
  this._ += strings[0];
  for (let i = 1, n = strings.length; i < n; ++i) {
    this._ += arguments[i] + strings[i];
  }
}
function appendRound(digits) {
  let d = Math.floor(digits);
  if (!(d >= 0)) throw new Error(`invalid digits: ${digits}`);
  if (d > 15) return append;
  const k = 10 ** d;
  return function(strings) {
    this._ += strings[0];
    for (let i = 1, n = strings.length; i < n; ++i) {
      this._ += Math.round(arguments[i] * k) / k + strings[i];
    }
  };
}
var Path = class {
  constructor(digits) {
    this._x0 = this._y0 = // start of current subpath
    this._x1 = this._y1 = null;
    this._ = "";
    this._append = digits == null ? append : appendRound(digits);
  }
  moveTo(x2, y2) {
    this._append`M${this._x0 = this._x1 = +x2},${this._y0 = this._y1 = +y2}`;
  }
  closePath() {
    if (this._x1 !== null) {
      this._x1 = this._x0, this._y1 = this._y0;
      this._append`Z`;
    }
  }
  lineTo(x2, y2) {
    this._append`L${this._x1 = +x2},${this._y1 = +y2}`;
  }
  quadraticCurveTo(x1, y1, x2, y2) {
    this._append`Q${+x1},${+y1},${this._x1 = +x2},${this._y1 = +y2}`;
  }
  bezierCurveTo(x1, y1, x2, y2, x3, y3) {
    this._append`C${+x1},${+y1},${+x2},${+y2},${this._x1 = +x3},${this._y1 = +y3}`;
  }
  arcTo(x1, y1, x2, y2, r) {
    x1 = +x1, y1 = +y1, x2 = +x2, y2 = +y2, r = +r;
    if (r < 0) throw new Error(`negative radius: ${r}`);
    let x0 = this._x1, y0 = this._y1, x21 = x2 - x1, y21 = y2 - y1, x01 = x0 - x1, y01 = y0 - y1, l01_2 = x01 * x01 + y01 * y01;
    if (this._x1 === null) {
      this._append`M${this._x1 = x1},${this._y1 = y1}`;
    } else if (!(l01_2 > epsilon3)) ;
    else if (!(Math.abs(y01 * x21 - y21 * x01) > epsilon3) || !r) {
      this._append`L${this._x1 = x1},${this._y1 = y1}`;
    } else {
      let x20 = x2 - x0, y20 = y2 - y0, l21_2 = x21 * x21 + y21 * y21, l20_2 = x20 * x20 + y20 * y20, l21 = Math.sqrt(l21_2), l01 = Math.sqrt(l01_2), l = r * Math.tan((pi - Math.acos((l21_2 + l01_2 - l20_2) / (2 * l21 * l01))) / 2), t01 = l / l01, t21 = l / l21;
      if (Math.abs(t01 - 1) > epsilon3) {
        this._append`L${x1 + t01 * x01},${y1 + t01 * y01}`;
      }
      this._append`A${r},${r},0,0,${+(y01 * x20 > x01 * y20)},${this._x1 = x1 + t21 * x21},${this._y1 = y1 + t21 * y21}`;
    }
  }
  arc(x2, y2, r, a0, a1, ccw) {
    x2 = +x2, y2 = +y2, r = +r, ccw = !!ccw;
    if (r < 0) throw new Error(`negative radius: ${r}`);
    let dx = r * Math.cos(a0), dy = r * Math.sin(a0), x0 = x2 + dx, y0 = y2 + dy, cw = 1 ^ ccw, da = ccw ? a0 - a1 : a1 - a0;
    if (this._x1 === null) {
      this._append`M${x0},${y0}`;
    } else if (Math.abs(this._x1 - x0) > epsilon3 || Math.abs(this._y1 - y0) > epsilon3) {
      this._append`L${x0},${y0}`;
    }
    if (!r) return;
    if (da < 0) da = da % tau + tau;
    if (da > tauEpsilon) {
      this._append`A${r},${r},0,1,${cw},${x2 - dx},${y2 - dy}A${r},${r},0,1,${cw},${this._x1 = x0},${this._y1 = y0}`;
    } else if (da > epsilon3) {
      this._append`A${r},${r},0,${+(da >= pi)},${cw},${this._x1 = x2 + r * Math.cos(a1)},${this._y1 = y2 + r * Math.sin(a1)}`;
    }
  }
  rect(x2, y2, w, h) {
    this._append`M${this._x0 = this._x1 = +x2},${this._y0 = this._y1 = +y2}h${w = +w}v${+h}h${-w}Z`;
  }
  toString() {
    return this._;
  }
};
function path() {
  return new Path();
}
path.prototype = Path.prototype;

// node_modules/d3-format/src/formatDecimal.js
function formatDecimal_default(x2) {
  return Math.abs(x2 = Math.round(x2)) >= 1e21 ? x2.toLocaleString("en").replace(/,/g, "") : x2.toString(10);
}
function formatDecimalParts(x2, p) {
  if ((i = (x2 = p ? x2.toExponential(p - 1) : x2.toExponential()).indexOf("e")) < 0) return null;
  var i, coefficient = x2.slice(0, i);
  return [
    coefficient.length > 1 ? coefficient[0] + coefficient.slice(2) : coefficient,
    +x2.slice(i + 1)
  ];
}

// node_modules/d3-format/src/exponent.js
function exponent_default(x2) {
  return x2 = formatDecimalParts(Math.abs(x2)), x2 ? x2[1] : NaN;
}

// node_modules/d3-format/src/formatGroup.js
function formatGroup_default(grouping, thousands) {
  return function(value, width) {
    var i = value.length, t = [], j = 0, g = grouping[0], length = 0;
    while (i > 0 && g > 0) {
      if (length + g + 1 > width) g = Math.max(1, width - length);
      t.push(value.substring(i -= g, i + g));
      if ((length += g + 1) > width) break;
      g = grouping[j = (j + 1) % grouping.length];
    }
    return t.reverse().join(thousands);
  };
}

// node_modules/d3-format/src/formatNumerals.js
function formatNumerals_default(numerals) {
  return function(value) {
    return value.replace(/[0-9]/g, function(i) {
      return numerals[+i];
    });
  };
}

// node_modules/d3-format/src/formatSpecifier.js
var re = /^(?:(.)?([<>=^]))?([+\-( ])?([$#])?(0)?(\d+)?(,)?(\.\d+)?(~)?([a-z%])?$/i;
function formatSpecifier(specifier) {
  if (!(match = re.exec(specifier))) throw new Error("invalid format: " + specifier);
  var match;
  return new FormatSpecifier({
    fill: match[1],
    align: match[2],
    sign: match[3],
    symbol: match[4],
    zero: match[5],
    width: match[6],
    comma: match[7],
    precision: match[8] && match[8].slice(1),
    trim: match[9],
    type: match[10]
  });
}
formatSpecifier.prototype = FormatSpecifier.prototype;
function FormatSpecifier(specifier) {
  this.fill = specifier.fill === void 0 ? " " : specifier.fill + "";
  this.align = specifier.align === void 0 ? ">" : specifier.align + "";
  this.sign = specifier.sign === void 0 ? "-" : specifier.sign + "";
  this.symbol = specifier.symbol === void 0 ? "" : specifier.symbol + "";
  this.zero = !!specifier.zero;
  this.width = specifier.width === void 0 ? void 0 : +specifier.width;
  this.comma = !!specifier.comma;
  this.precision = specifier.precision === void 0 ? void 0 : +specifier.precision;
  this.trim = !!specifier.trim;
  this.type = specifier.type === void 0 ? "" : specifier.type + "";
}
FormatSpecifier.prototype.toString = function() {
  return this.fill + this.align + this.sign + this.symbol + (this.zero ? "0" : "") + (this.width === void 0 ? "" : Math.max(1, this.width | 0)) + (this.comma ? "," : "") + (this.precision === void 0 ? "" : "." + Math.max(0, this.precision | 0)) + (this.trim ? "~" : "") + this.type;
};

// node_modules/d3-format/src/formatTrim.js
function formatTrim_default(s) {
  out: for (var n = s.length, i = 1, i0 = -1, i1; i < n; ++i) {
    switch (s[i]) {
      case ".":
        i0 = i1 = i;
        break;
      case "0":
        if (i0 === 0) i0 = i;
        i1 = i;
        break;
      default:
        if (!+s[i]) break out;
        if (i0 > 0) i0 = 0;
        break;
    }
  }
  return i0 > 0 ? s.slice(0, i0) + s.slice(i1 + 1) : s;
}

// node_modules/d3-format/src/formatPrefixAuto.js
var prefixExponent;
function formatPrefixAuto_default(x2, p) {
  var d = formatDecimalParts(x2, p);
  if (!d) return x2 + "";
  var coefficient = d[0], exponent = d[1], i = exponent - (prefixExponent = Math.max(-8, Math.min(8, Math.floor(exponent / 3))) * 3) + 1, n = coefficient.length;
  return i === n ? coefficient : i > n ? coefficient + new Array(i - n + 1).join("0") : i > 0 ? coefficient.slice(0, i) + "." + coefficient.slice(i) : "0." + new Array(1 - i).join("0") + formatDecimalParts(x2, Math.max(0, p + i - 1))[0];
}

// node_modules/d3-format/src/formatRounded.js
function formatRounded_default(x2, p) {
  var d = formatDecimalParts(x2, p);
  if (!d) return x2 + "";
  var coefficient = d[0], exponent = d[1];
  return exponent < 0 ? "0." + new Array(-exponent).join("0") + coefficient : coefficient.length > exponent + 1 ? coefficient.slice(0, exponent + 1) + "." + coefficient.slice(exponent + 1) : coefficient + new Array(exponent - coefficient.length + 2).join("0");
}

// node_modules/d3-format/src/formatTypes.js
var formatTypes_default = {
  "%": (x2, p) => (x2 * 100).toFixed(p),
  "b": (x2) => Math.round(x2).toString(2),
  "c": (x2) => x2 + "",
  "d": formatDecimal_default,
  "e": (x2, p) => x2.toExponential(p),
  "f": (x2, p) => x2.toFixed(p),
  "g": (x2, p) => x2.toPrecision(p),
  "o": (x2) => Math.round(x2).toString(8),
  "p": (x2, p) => formatRounded_default(x2 * 100, p),
  "r": formatRounded_default,
  "s": formatPrefixAuto_default,
  "X": (x2) => Math.round(x2).toString(16).toUpperCase(),
  "x": (x2) => Math.round(x2).toString(16)
};

// node_modules/d3-format/src/identity.js
function identity_default2(x2) {
  return x2;
}

// node_modules/d3-format/src/locale.js
var map = Array.prototype.map;
var prefixes = ["y", "z", "a", "f", "p", "n", "\xB5", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y"];
function locale_default(locale3) {
  var group = locale3.grouping === void 0 || locale3.thousands === void 0 ? identity_default2 : formatGroup_default(map.call(locale3.grouping, Number), locale3.thousands + ""), currencyPrefix = locale3.currency === void 0 ? "" : locale3.currency[0] + "", currencySuffix = locale3.currency === void 0 ? "" : locale3.currency[1] + "", decimal = locale3.decimal === void 0 ? "." : locale3.decimal + "", numerals = locale3.numerals === void 0 ? identity_default2 : formatNumerals_default(map.call(locale3.numerals, String)), percent = locale3.percent === void 0 ? "%" : locale3.percent + "", minus = locale3.minus === void 0 ? "\u2212" : locale3.minus + "", nan = locale3.nan === void 0 ? "NaN" : locale3.nan + "";
  function newFormat(specifier) {
    specifier = formatSpecifier(specifier);
    var fill = specifier.fill, align = specifier.align, sign = specifier.sign, symbol = specifier.symbol, zero3 = specifier.zero, width = specifier.width, comma = specifier.comma, precision = specifier.precision, trim = specifier.trim, type2 = specifier.type;
    if (type2 === "n") comma = true, type2 = "g";
    else if (!formatTypes_default[type2]) precision === void 0 && (precision = 12), trim = true, type2 = "g";
    if (zero3 || fill === "0" && align === "=") zero3 = true, fill = "0", align = "=";
    var prefix = symbol === "$" ? currencyPrefix : symbol === "#" && /[boxX]/.test(type2) ? "0" + type2.toLowerCase() : "", suffix = symbol === "$" ? currencySuffix : /[%p]/.test(type2) ? percent : "";
    var formatType = formatTypes_default[type2], maybeSuffix = /[defgprs%]/.test(type2);
    precision = precision === void 0 ? 6 : /[gprs]/.test(type2) ? Math.max(1, Math.min(21, precision)) : Math.max(0, Math.min(20, precision));
    function format2(value) {
      var valuePrefix = prefix, valueSuffix = suffix, i, n, c;
      if (type2 === "c") {
        valueSuffix = formatType(value) + valueSuffix;
        value = "";
      } else {
        value = +value;
        var valueNegative = value < 0 || 1 / value < 0;
        value = isNaN(value) ? nan : formatType(Math.abs(value), precision);
        if (trim) value = formatTrim_default(value);
        if (valueNegative && +value === 0 && sign !== "+") valueNegative = false;
        valuePrefix = (valueNegative ? sign === "(" ? sign : minus : sign === "-" || sign === "(" ? "" : sign) + valuePrefix;
        valueSuffix = (type2 === "s" ? prefixes[8 + prefixExponent / 3] : "") + valueSuffix + (valueNegative && sign === "(" ? ")" : "");
        if (maybeSuffix) {
          i = -1, n = value.length;
          while (++i < n) {
            if (c = value.charCodeAt(i), 48 > c || c > 57) {
              valueSuffix = (c === 46 ? decimal + value.slice(i + 1) : value.slice(i)) + valueSuffix;
              value = value.slice(0, i);
              break;
            }
          }
        }
      }
      if (comma && !zero3) value = group(value, Infinity);
      var length = valuePrefix.length + value.length + valueSuffix.length, padding = length < width ? new Array(width - length + 1).join(fill) : "";
      if (comma && zero3) value = group(padding + value, padding.length ? width - valueSuffix.length : Infinity), padding = "";
      switch (align) {
        case "<":
          value = valuePrefix + value + valueSuffix + padding;
          break;
        case "=":
          value = valuePrefix + padding + value + valueSuffix;
          break;
        case "^":
          value = padding.slice(0, length = padding.length >> 1) + valuePrefix + value + valueSuffix + padding.slice(length);
          break;
        default:
          value = padding + valuePrefix + value + valueSuffix;
          break;
      }
      return numerals(value);
    }
    format2.toString = function() {
      return specifier + "";
    };
    return format2;
  }
  function formatPrefix2(specifier, value) {
    var f = newFormat((specifier = formatSpecifier(specifier), specifier.type = "f", specifier)), e = Math.max(-8, Math.min(8, Math.floor(exponent_default(value) / 3))) * 3, k = Math.pow(10, -e), prefix = prefixes[8 + e / 3];
    return function(value2) {
      return f(k * value2) + prefix;
    };
  }
  return {
    format: newFormat,
    formatPrefix: formatPrefix2
  };
}

// node_modules/d3-format/src/defaultLocale.js
var locale;
var format;
var formatPrefix;
defaultLocale({
  thousands: ",",
  grouping: [3],
  currency: ["$", ""]
});
function defaultLocale(definition) {
  locale = locale_default(definition);
  format = locale.format;
  formatPrefix = locale.formatPrefix;
  return locale;
}

// node_modules/d3-format/src/precisionFixed.js
function precisionFixed_default(step) {
  return Math.max(0, -exponent_default(Math.abs(step)));
}

// node_modules/d3-format/src/precisionPrefix.js
function precisionPrefix_default(step, value) {
  return Math.max(0, Math.max(-8, Math.min(8, Math.floor(exponent_default(value) / 3))) * 3 - exponent_default(Math.abs(step)));
}

// node_modules/d3-format/src/precisionRound.js
function precisionRound_default(step, max2) {
  step = Math.abs(step), max2 = Math.abs(max2) - step;
  return Math.max(0, exponent_default(max2) - exponent_default(step)) + 1;
}

// node_modules/d3-scale/src/init.js
function initRange(domain, range) {
  switch (arguments.length) {
    case 0:
      break;
    case 1:
      this.range(domain);
      break;
    default:
      this.range(range).domain(domain);
      break;
  }
  return this;
}

// node_modules/d3-scale/src/constant.js
function constants(x2) {
  return function() {
    return x2;
  };
}

// node_modules/d3-scale/src/number.js
function number3(x2) {
  return +x2;
}

// node_modules/d3-scale/src/continuous.js
var unit = [0, 1];
function identity2(x2) {
  return x2;
}
function normalize(a, b) {
  return (b -= a = +a) ? function(x2) {
    return (x2 - a) / b;
  } : constants(isNaN(b) ? NaN : 0.5);
}
function clamper(a, b) {
  var t;
  if (a > b) t = a, a = b, b = t;
  return function(x2) {
    return Math.max(a, Math.min(b, x2));
  };
}
function bimap(domain, range, interpolate) {
  var d0 = domain[0], d1 = domain[1], r0 = range[0], r1 = range[1];
  if (d1 < d0) d0 = normalize(d1, d0), r0 = interpolate(r1, r0);
  else d0 = normalize(d0, d1), r0 = interpolate(r0, r1);
  return function(x2) {
    return r0(d0(x2));
  };
}
function polymap(domain, range, interpolate) {
  var j = Math.min(domain.length, range.length) - 1, d = new Array(j), r = new Array(j), i = -1;
  if (domain[j] < domain[0]) {
    domain = domain.slice().reverse();
    range = range.slice().reverse();
  }
  while (++i < j) {
    d[i] = normalize(domain[i], domain[i + 1]);
    r[i] = interpolate(range[i], range[i + 1]);
  }
  return function(x2) {
    var i2 = bisect_default(domain, x2, 1, j) - 1;
    return r[i2](d[i2](x2));
  };
}
function copy(source, target) {
  return target.domain(source.domain()).range(source.range()).interpolate(source.interpolate()).clamp(source.clamp()).unknown(source.unknown());
}
function transformer() {
  var domain = unit, range = unit, interpolate = value_default, transform2, untransform, unknown, clamp = identity2, piecewise, output, input;
  function rescale() {
    var n = Math.min(domain.length, range.length);
    if (clamp !== identity2) clamp = clamper(domain[0], domain[n - 1]);
    piecewise = n > 2 ? polymap : bimap;
    output = input = null;
    return scale;
  }
  function scale(x2) {
    return x2 == null || isNaN(x2 = +x2) ? unknown : (output || (output = piecewise(domain.map(transform2), range, interpolate)))(transform2(clamp(x2)));
  }
  scale.invert = function(y2) {
    return clamp(untransform((input || (input = piecewise(range, domain.map(transform2), number_default)))(y2)));
  };
  scale.domain = function(_) {
    return arguments.length ? (domain = Array.from(_, number3), rescale()) : domain.slice();
  };
  scale.range = function(_) {
    return arguments.length ? (range = Array.from(_), rescale()) : range.slice();
  };
  scale.rangeRound = function(_) {
    return range = Array.from(_), interpolate = round_default, rescale();
  };
  scale.clamp = function(_) {
    return arguments.length ? (clamp = _ ? true : identity2, rescale()) : clamp !== identity2;
  };
  scale.interpolate = function(_) {
    return arguments.length ? (interpolate = _, rescale()) : interpolate;
  };
  scale.unknown = function(_) {
    return arguments.length ? (unknown = _, scale) : unknown;
  };
  return function(t, u) {
    transform2 = t, untransform = u;
    return rescale();
  };
}
function continuous() {
  return transformer()(identity2, identity2);
}

// node_modules/d3-scale/src/tickFormat.js
function tickFormat(start2, stop, count, specifier) {
  var step = tickStep(start2, stop, count), precision;
  specifier = formatSpecifier(specifier == null ? ",f" : specifier);
  switch (specifier.type) {
    case "s": {
      var value = Math.max(Math.abs(start2), Math.abs(stop));
      if (specifier.precision == null && !isNaN(precision = precisionPrefix_default(step, value))) specifier.precision = precision;
      return formatPrefix(specifier, value);
    }
    case "":
    case "e":
    case "g":
    case "p":
    case "r": {
      if (specifier.precision == null && !isNaN(precision = precisionRound_default(step, Math.max(Math.abs(start2), Math.abs(stop))))) specifier.precision = precision - (specifier.type === "e");
      break;
    }
    case "f":
    case "%": {
      if (specifier.precision == null && !isNaN(precision = precisionFixed_default(step))) specifier.precision = precision - (specifier.type === "%") * 2;
      break;
    }
  }
  return format(specifier);
}

// node_modules/d3-scale/src/linear.js
function linearish(scale) {
  var domain = scale.domain;
  scale.ticks = function(count) {
    var d = domain();
    return ticks(d[0], d[d.length - 1], count == null ? 10 : count);
  };
  scale.tickFormat = function(count, specifier) {
    var d = domain();
    return tickFormat(d[0], d[d.length - 1], count == null ? 10 : count, specifier);
  };
  scale.nice = function(count) {
    if (count == null) count = 10;
    var d = domain();
    var i0 = 0;
    var i1 = d.length - 1;
    var start2 = d[i0];
    var stop = d[i1];
    var prestep;
    var step;
    var maxIter = 10;
    if (stop < start2) {
      step = start2, start2 = stop, stop = step;
      step = i0, i0 = i1, i1 = step;
    }
    while (maxIter-- > 0) {
      step = tickIncrement(start2, stop, count);
      if (step === prestep) {
        d[i0] = start2;
        d[i1] = stop;
        return domain(d);
      } else if (step > 0) {
        start2 = Math.floor(start2 / step) * step;
        stop = Math.ceil(stop / step) * step;
      } else if (step < 0) {
        start2 = Math.ceil(start2 * step) / step;
        stop = Math.floor(stop * step) / step;
      } else {
        break;
      }
      prestep = step;
    }
    return scale;
  };
  return scale;
}
function linear2() {
  var scale = continuous();
  scale.copy = function() {
    return copy(scale, linear2());
  };
  initRange.apply(scale, arguments);
  return linearish(scale);
}

// node_modules/d3-scale/src/nice.js
function nice(domain, interval2) {
  domain = domain.slice();
  var i0 = 0, i1 = domain.length - 1, x0 = domain[i0], x1 = domain[i1], t;
  if (x1 < x0) {
    t = i0, i0 = i1, i1 = t;
    t = x0, x0 = x1, x1 = t;
  }
  domain[i0] = interval2.floor(x0);
  domain[i1] = interval2.ceil(x1);
  return domain;
}

// node_modules/d3-time/src/interval.js
var t0 = /* @__PURE__ */ new Date();
var t1 = /* @__PURE__ */ new Date();
function timeInterval(floori, offseti, count, field) {
  function interval2(date2) {
    return floori(date2 = arguments.length === 0 ? /* @__PURE__ */ new Date() : /* @__PURE__ */ new Date(+date2)), date2;
  }
  interval2.floor = (date2) => {
    return floori(date2 = /* @__PURE__ */ new Date(+date2)), date2;
  };
  interval2.ceil = (date2) => {
    return floori(date2 = new Date(date2 - 1)), offseti(date2, 1), floori(date2), date2;
  };
  interval2.round = (date2) => {
    const d0 = interval2(date2), d1 = interval2.ceil(date2);
    return date2 - d0 < d1 - date2 ? d0 : d1;
  };
  interval2.offset = (date2, step) => {
    return offseti(date2 = /* @__PURE__ */ new Date(+date2), step == null ? 1 : Math.floor(step)), date2;
  };
  interval2.range = (start2, stop, step) => {
    const range = [];
    start2 = interval2.ceil(start2);
    step = step == null ? 1 : Math.floor(step);
    if (!(start2 < stop) || !(step > 0)) return range;
    let previous;
    do
      range.push(previous = /* @__PURE__ */ new Date(+start2)), offseti(start2, step), floori(start2);
    while (previous < start2 && start2 < stop);
    return range;
  };
  interval2.filter = (test) => {
    return timeInterval((date2) => {
      if (date2 >= date2) while (floori(date2), !test(date2)) date2.setTime(date2 - 1);
    }, (date2, step) => {
      if (date2 >= date2) {
        if (step < 0) while (++step <= 0) {
          while (offseti(date2, -1), !test(date2)) {
          }
        }
        else while (--step >= 0) {
          while (offseti(date2, 1), !test(date2)) {
          }
        }
      }
    });
  };
  if (count) {
    interval2.count = (start2, end) => {
      t0.setTime(+start2), t1.setTime(+end);
      floori(t0), floori(t1);
      return Math.floor(count(t0, t1));
    };
    interval2.every = (step) => {
      step = Math.floor(step);
      return !isFinite(step) || !(step > 0) ? null : !(step > 1) ? interval2 : interval2.filter(field ? (d) => field(d) % step === 0 : (d) => interval2.count(0, d) % step === 0);
    };
  }
  return interval2;
}

// node_modules/d3-time/src/millisecond.js
var millisecond = timeInterval(() => {
}, (date2, step) => {
  date2.setTime(+date2 + step);
}, (start2, end) => {
  return end - start2;
});
millisecond.every = (k) => {
  k = Math.floor(k);
  if (!isFinite(k) || !(k > 0)) return null;
  if (!(k > 1)) return millisecond;
  return timeInterval((date2) => {
    date2.setTime(Math.floor(date2 / k) * k);
  }, (date2, step) => {
    date2.setTime(+date2 + step * k);
  }, (start2, end) => {
    return (end - start2) / k;
  });
};
var milliseconds = millisecond.range;

// node_modules/d3-time/src/duration.js
var durationSecond = 1e3;
var durationMinute = durationSecond * 60;
var durationHour = durationMinute * 60;
var durationDay = durationHour * 24;
var durationWeek = durationDay * 7;
var durationMonth = durationDay * 30;
var durationYear = durationDay * 365;

// node_modules/d3-time/src/second.js
var second = timeInterval((date2) => {
  date2.setTime(date2 - date2.getMilliseconds());
}, (date2, step) => {
  date2.setTime(+date2 + step * durationSecond);
}, (start2, end) => {
  return (end - start2) / durationSecond;
}, (date2) => {
  return date2.getUTCSeconds();
});
var seconds = second.range;

// node_modules/d3-time/src/minute.js
var timeMinute = timeInterval((date2) => {
  date2.setTime(date2 - date2.getMilliseconds() - date2.getSeconds() * durationSecond);
}, (date2, step) => {
  date2.setTime(+date2 + step * durationMinute);
}, (start2, end) => {
  return (end - start2) / durationMinute;
}, (date2) => {
  return date2.getMinutes();
});
var timeMinutes = timeMinute.range;
var utcMinute = timeInterval((date2) => {
  date2.setUTCSeconds(0, 0);
}, (date2, step) => {
  date2.setTime(+date2 + step * durationMinute);
}, (start2, end) => {
  return (end - start2) / durationMinute;
}, (date2) => {
  return date2.getUTCMinutes();
});
var utcMinutes = utcMinute.range;

// node_modules/d3-time/src/hour.js
var timeHour = timeInterval((date2) => {
  date2.setTime(date2 - date2.getMilliseconds() - date2.getSeconds() * durationSecond - date2.getMinutes() * durationMinute);
}, (date2, step) => {
  date2.setTime(+date2 + step * durationHour);
}, (start2, end) => {
  return (end - start2) / durationHour;
}, (date2) => {
  return date2.getHours();
});
var timeHours = timeHour.range;
var utcHour = timeInterval((date2) => {
  date2.setUTCMinutes(0, 0, 0);
}, (date2, step) => {
  date2.setTime(+date2 + step * durationHour);
}, (start2, end) => {
  return (end - start2) / durationHour;
}, (date2) => {
  return date2.getUTCHours();
});
var utcHours = utcHour.range;

// node_modules/d3-time/src/day.js
var timeDay = timeInterval(
  (date2) => date2.setHours(0, 0, 0, 0),
  (date2, step) => date2.setDate(date2.getDate() + step),
  (start2, end) => (end - start2 - (end.getTimezoneOffset() - start2.getTimezoneOffset()) * durationMinute) / durationDay,
  (date2) => date2.getDate() - 1
);
var timeDays = timeDay.range;
var utcDay = timeInterval((date2) => {
  date2.setUTCHours(0, 0, 0, 0);
}, (date2, step) => {
  date2.setUTCDate(date2.getUTCDate() + step);
}, (start2, end) => {
  return (end - start2) / durationDay;
}, (date2) => {
  return date2.getUTCDate() - 1;
});
var utcDays = utcDay.range;
var unixDay = timeInterval((date2) => {
  date2.setUTCHours(0, 0, 0, 0);
}, (date2, step) => {
  date2.setUTCDate(date2.getUTCDate() + step);
}, (start2, end) => {
  return (end - start2) / durationDay;
}, (date2) => {
  return Math.floor(date2 / durationDay);
});
var unixDays = unixDay.range;

// node_modules/d3-time/src/week.js
function timeWeekday(i) {
  return timeInterval((date2) => {
    date2.setDate(date2.getDate() - (date2.getDay() + 7 - i) % 7);
    date2.setHours(0, 0, 0, 0);
  }, (date2, step) => {
    date2.setDate(date2.getDate() + step * 7);
  }, (start2, end) => {
    return (end - start2 - (end.getTimezoneOffset() - start2.getTimezoneOffset()) * durationMinute) / durationWeek;
  });
}
var timeSunday = timeWeekday(0);
var timeMonday = timeWeekday(1);
var timeTuesday = timeWeekday(2);
var timeWednesday = timeWeekday(3);
var timeThursday = timeWeekday(4);
var timeFriday = timeWeekday(5);
var timeSaturday = timeWeekday(6);
var timeSundays = timeSunday.range;
var timeMondays = timeMonday.range;
var timeTuesdays = timeTuesday.range;
var timeWednesdays = timeWednesday.range;
var timeThursdays = timeThursday.range;
var timeFridays = timeFriday.range;
var timeSaturdays = timeSaturday.range;
function utcWeekday(i) {
  return timeInterval((date2) => {
    date2.setUTCDate(date2.getUTCDate() - (date2.getUTCDay() + 7 - i) % 7);
    date2.setUTCHours(0, 0, 0, 0);
  }, (date2, step) => {
    date2.setUTCDate(date2.getUTCDate() + step * 7);
  }, (start2, end) => {
    return (end - start2) / durationWeek;
  });
}
var utcSunday = utcWeekday(0);
var utcMonday = utcWeekday(1);
var utcTuesday = utcWeekday(2);
var utcWednesday = utcWeekday(3);
var utcThursday = utcWeekday(4);
var utcFriday = utcWeekday(5);
var utcSaturday = utcWeekday(6);
var utcSundays = utcSunday.range;
var utcMondays = utcMonday.range;
var utcTuesdays = utcTuesday.range;
var utcWednesdays = utcWednesday.range;
var utcThursdays = utcThursday.range;
var utcFridays = utcFriday.range;
var utcSaturdays = utcSaturday.range;

// node_modules/d3-time/src/month.js
var timeMonth = timeInterval((date2) => {
  date2.setDate(1);
  date2.setHours(0, 0, 0, 0);
}, (date2, step) => {
  date2.setMonth(date2.getMonth() + step);
}, (start2, end) => {
  return end.getMonth() - start2.getMonth() + (end.getFullYear() - start2.getFullYear()) * 12;
}, (date2) => {
  return date2.getMonth();
});
var timeMonths = timeMonth.range;
var utcMonth = timeInterval((date2) => {
  date2.setUTCDate(1);
  date2.setUTCHours(0, 0, 0, 0);
}, (date2, step) => {
  date2.setUTCMonth(date2.getUTCMonth() + step);
}, (start2, end) => {
  return end.getUTCMonth() - start2.getUTCMonth() + (end.getUTCFullYear() - start2.getUTCFullYear()) * 12;
}, (date2) => {
  return date2.getUTCMonth();
});
var utcMonths = utcMonth.range;

// node_modules/d3-time/src/year.js
var timeYear = timeInterval((date2) => {
  date2.setMonth(0, 1);
  date2.setHours(0, 0, 0, 0);
}, (date2, step) => {
  date2.setFullYear(date2.getFullYear() + step);
}, (start2, end) => {
  return end.getFullYear() - start2.getFullYear();
}, (date2) => {
  return date2.getFullYear();
});
timeYear.every = (k) => {
  return !isFinite(k = Math.floor(k)) || !(k > 0) ? null : timeInterval((date2) => {
    date2.setFullYear(Math.floor(date2.getFullYear() / k) * k);
    date2.setMonth(0, 1);
    date2.setHours(0, 0, 0, 0);
  }, (date2, step) => {
    date2.setFullYear(date2.getFullYear() + step * k);
  });
};
var timeYears = timeYear.range;
var utcYear = timeInterval((date2) => {
  date2.setUTCMonth(0, 1);
  date2.setUTCHours(0, 0, 0, 0);
}, (date2, step) => {
  date2.setUTCFullYear(date2.getUTCFullYear() + step);
}, (start2, end) => {
  return end.getUTCFullYear() - start2.getUTCFullYear();
}, (date2) => {
  return date2.getUTCFullYear();
});
utcYear.every = (k) => {
  return !isFinite(k = Math.floor(k)) || !(k > 0) ? null : timeInterval((date2) => {
    date2.setUTCFullYear(Math.floor(date2.getUTCFullYear() / k) * k);
    date2.setUTCMonth(0, 1);
    date2.setUTCHours(0, 0, 0, 0);
  }, (date2, step) => {
    date2.setUTCFullYear(date2.getUTCFullYear() + step * k);
  });
};
var utcYears = utcYear.range;

// node_modules/d3-time/src/ticks.js
function ticker(year, month, week, day, hour, minute) {
  const tickIntervals = [
    [second, 1, durationSecond],
    [second, 5, 5 * durationSecond],
    [second, 15, 15 * durationSecond],
    [second, 30, 30 * durationSecond],
    [minute, 1, durationMinute],
    [minute, 5, 5 * durationMinute],
    [minute, 15, 15 * durationMinute],
    [minute, 30, 30 * durationMinute],
    [hour, 1, durationHour],
    [hour, 3, 3 * durationHour],
    [hour, 6, 6 * durationHour],
    [hour, 12, 12 * durationHour],
    [day, 1, durationDay],
    [day, 2, 2 * durationDay],
    [week, 1, durationWeek],
    [month, 1, durationMonth],
    [month, 3, 3 * durationMonth],
    [year, 1, durationYear]
  ];
  function ticks2(start2, stop, count) {
    const reverse = stop < start2;
    if (reverse) [start2, stop] = [stop, start2];
    const interval2 = count && typeof count.range === "function" ? count : tickInterval(start2, stop, count);
    const ticks3 = interval2 ? interval2.range(start2, +stop + 1) : [];
    return reverse ? ticks3.reverse() : ticks3;
  }
  function tickInterval(start2, stop, count) {
    const target = Math.abs(stop - start2) / count;
    const i = bisector(([, , step2]) => step2).right(tickIntervals, target);
    if (i === tickIntervals.length) return year.every(tickStep(start2 / durationYear, stop / durationYear, count));
    if (i === 0) return millisecond.every(Math.max(tickStep(start2, stop, count), 1));
    const [t, step] = tickIntervals[target / tickIntervals[i - 1][2] < tickIntervals[i][2] / target ? i - 1 : i];
    return t.every(step);
  }
  return [ticks2, tickInterval];
}
var [utcTicks, utcTickInterval] = ticker(utcYear, utcMonth, utcSunday, unixDay, utcHour, utcMinute);
var [timeTicks, timeTickInterval] = ticker(timeYear, timeMonth, timeSunday, timeDay, timeHour, timeMinute);

// node_modules/d3-time-format/src/locale.js
function localDate(d) {
  if (0 <= d.y && d.y < 100) {
    var date2 = new Date(-1, d.m, d.d, d.H, d.M, d.S, d.L);
    date2.setFullYear(d.y);
    return date2;
  }
  return new Date(d.y, d.m, d.d, d.H, d.M, d.S, d.L);
}
function utcDate(d) {
  if (0 <= d.y && d.y < 100) {
    var date2 = new Date(Date.UTC(-1, d.m, d.d, d.H, d.M, d.S, d.L));
    date2.setUTCFullYear(d.y);
    return date2;
  }
  return new Date(Date.UTC(d.y, d.m, d.d, d.H, d.M, d.S, d.L));
}
function newDate(y2, m, d) {
  return { y: y2, m, d, H: 0, M: 0, S: 0, L: 0 };
}
function formatLocale(locale3) {
  var locale_dateTime = locale3.dateTime, locale_date = locale3.date, locale_time = locale3.time, locale_periods = locale3.periods, locale_weekdays = locale3.days, locale_shortWeekdays = locale3.shortDays, locale_months = locale3.months, locale_shortMonths = locale3.shortMonths;
  var periodRe = formatRe(locale_periods), periodLookup = formatLookup(locale_periods), weekdayRe = formatRe(locale_weekdays), weekdayLookup = formatLookup(locale_weekdays), shortWeekdayRe = formatRe(locale_shortWeekdays), shortWeekdayLookup = formatLookup(locale_shortWeekdays), monthRe = formatRe(locale_months), monthLookup = formatLookup(locale_months), shortMonthRe = formatRe(locale_shortMonths), shortMonthLookup = formatLookup(locale_shortMonths);
  var formats = {
    "a": formatShortWeekday,
    "A": formatWeekday,
    "b": formatShortMonth,
    "B": formatMonth,
    "c": null,
    "d": formatDayOfMonth,
    "e": formatDayOfMonth,
    "f": formatMicroseconds,
    "g": formatYearISO,
    "G": formatFullYearISO,
    "H": formatHour24,
    "I": formatHour12,
    "j": formatDayOfYear,
    "L": formatMilliseconds,
    "m": formatMonthNumber,
    "M": formatMinutes,
    "p": formatPeriod,
    "q": formatQuarter,
    "Q": formatUnixTimestamp,
    "s": formatUnixTimestampSeconds,
    "S": formatSeconds,
    "u": formatWeekdayNumberMonday,
    "U": formatWeekNumberSunday,
    "V": formatWeekNumberISO,
    "w": formatWeekdayNumberSunday,
    "W": formatWeekNumberMonday,
    "x": null,
    "X": null,
    "y": formatYear,
    "Y": formatFullYear,
    "Z": formatZone,
    "%": formatLiteralPercent
  };
  var utcFormats = {
    "a": formatUTCShortWeekday,
    "A": formatUTCWeekday,
    "b": formatUTCShortMonth,
    "B": formatUTCMonth,
    "c": null,
    "d": formatUTCDayOfMonth,
    "e": formatUTCDayOfMonth,
    "f": formatUTCMicroseconds,
    "g": formatUTCYearISO,
    "G": formatUTCFullYearISO,
    "H": formatUTCHour24,
    "I": formatUTCHour12,
    "j": formatUTCDayOfYear,
    "L": formatUTCMilliseconds,
    "m": formatUTCMonthNumber,
    "M": formatUTCMinutes,
    "p": formatUTCPeriod,
    "q": formatUTCQuarter,
    "Q": formatUnixTimestamp,
    "s": formatUnixTimestampSeconds,
    "S": formatUTCSeconds,
    "u": formatUTCWeekdayNumberMonday,
    "U": formatUTCWeekNumberSunday,
    "V": formatUTCWeekNumberISO,
    "w": formatUTCWeekdayNumberSunday,
    "W": formatUTCWeekNumberMonday,
    "x": null,
    "X": null,
    "y": formatUTCYear,
    "Y": formatUTCFullYear,
    "Z": formatUTCZone,
    "%": formatLiteralPercent
  };
  var parses = {
    "a": parseShortWeekday,
    "A": parseWeekday,
    "b": parseShortMonth,
    "B": parseMonth,
    "c": parseLocaleDateTime,
    "d": parseDayOfMonth,
    "e": parseDayOfMonth,
    "f": parseMicroseconds,
    "g": parseYear,
    "G": parseFullYear,
    "H": parseHour24,
    "I": parseHour24,
    "j": parseDayOfYear,
    "L": parseMilliseconds,
    "m": parseMonthNumber,
    "M": parseMinutes,
    "p": parsePeriod,
    "q": parseQuarter,
    "Q": parseUnixTimestamp,
    "s": parseUnixTimestampSeconds,
    "S": parseSeconds,
    "u": parseWeekdayNumberMonday,
    "U": parseWeekNumberSunday,
    "V": parseWeekNumberISO,
    "w": parseWeekdayNumberSunday,
    "W": parseWeekNumberMonday,
    "x": parseLocaleDate,
    "X": parseLocaleTime,
    "y": parseYear,
    "Y": parseFullYear,
    "Z": parseZone,
    "%": parseLiteralPercent
  };
  formats.x = newFormat(locale_date, formats);
  formats.X = newFormat(locale_time, formats);
  formats.c = newFormat(locale_dateTime, formats);
  utcFormats.x = newFormat(locale_date, utcFormats);
  utcFormats.X = newFormat(locale_time, utcFormats);
  utcFormats.c = newFormat(locale_dateTime, utcFormats);
  function newFormat(specifier, formats2) {
    return function(date2) {
      var string = [], i = -1, j = 0, n = specifier.length, c, pad2, format2;
      if (!(date2 instanceof Date)) date2 = /* @__PURE__ */ new Date(+date2);
      while (++i < n) {
        if (specifier.charCodeAt(i) === 37) {
          string.push(specifier.slice(j, i));
          if ((pad2 = pads[c = specifier.charAt(++i)]) != null) c = specifier.charAt(++i);
          else pad2 = c === "e" ? " " : "0";
          if (format2 = formats2[c]) c = format2(date2, pad2);
          string.push(c);
          j = i + 1;
        }
      }
      string.push(specifier.slice(j, i));
      return string.join("");
    };
  }
  function newParse(specifier, Z) {
    return function(string) {
      var d = newDate(1900, void 0, 1), i = parseSpecifier(d, specifier, string += "", 0), week, day;
      if (i != string.length) return null;
      if ("Q" in d) return new Date(d.Q);
      if ("s" in d) return new Date(d.s * 1e3 + ("L" in d ? d.L : 0));
      if (Z && !("Z" in d)) d.Z = 0;
      if ("p" in d) d.H = d.H % 12 + d.p * 12;
      if (d.m === void 0) d.m = "q" in d ? d.q : 0;
      if ("V" in d) {
        if (d.V < 1 || d.V > 53) return null;
        if (!("w" in d)) d.w = 1;
        if ("Z" in d) {
          week = utcDate(newDate(d.y, 0, 1)), day = week.getUTCDay();
          week = day > 4 || day === 0 ? utcMonday.ceil(week) : utcMonday(week);
          week = utcDay.offset(week, (d.V - 1) * 7);
          d.y = week.getUTCFullYear();
          d.m = week.getUTCMonth();
          d.d = week.getUTCDate() + (d.w + 6) % 7;
        } else {
          week = localDate(newDate(d.y, 0, 1)), day = week.getDay();
          week = day > 4 || day === 0 ? timeMonday.ceil(week) : timeMonday(week);
          week = timeDay.offset(week, (d.V - 1) * 7);
          d.y = week.getFullYear();
          d.m = week.getMonth();
          d.d = week.getDate() + (d.w + 6) % 7;
        }
      } else if ("W" in d || "U" in d) {
        if (!("w" in d)) d.w = "u" in d ? d.u % 7 : "W" in d ? 1 : 0;
        day = "Z" in d ? utcDate(newDate(d.y, 0, 1)).getUTCDay() : localDate(newDate(d.y, 0, 1)).getDay();
        d.m = 0;
        d.d = "W" in d ? (d.w + 6) % 7 + d.W * 7 - (day + 5) % 7 : d.w + d.U * 7 - (day + 6) % 7;
      }
      if ("Z" in d) {
        d.H += d.Z / 100 | 0;
        d.M += d.Z % 100;
        return utcDate(d);
      }
      return localDate(d);
    };
  }
  function parseSpecifier(d, specifier, string, j) {
    var i = 0, n = specifier.length, m = string.length, c, parse;
    while (i < n) {
      if (j >= m) return -1;
      c = specifier.charCodeAt(i++);
      if (c === 37) {
        c = specifier.charAt(i++);
        parse = parses[c in pads ? specifier.charAt(i++) : c];
        if (!parse || (j = parse(d, string, j)) < 0) return -1;
      } else if (c != string.charCodeAt(j++)) {
        return -1;
      }
    }
    return j;
  }
  function parsePeriod(d, string, i) {
    var n = periodRe.exec(string.slice(i));
    return n ? (d.p = periodLookup.get(n[0].toLowerCase()), i + n[0].length) : -1;
  }
  function parseShortWeekday(d, string, i) {
    var n = shortWeekdayRe.exec(string.slice(i));
    return n ? (d.w = shortWeekdayLookup.get(n[0].toLowerCase()), i + n[0].length) : -1;
  }
  function parseWeekday(d, string, i) {
    var n = weekdayRe.exec(string.slice(i));
    return n ? (d.w = weekdayLookup.get(n[0].toLowerCase()), i + n[0].length) : -1;
  }
  function parseShortMonth(d, string, i) {
    var n = shortMonthRe.exec(string.slice(i));
    return n ? (d.m = shortMonthLookup.get(n[0].toLowerCase()), i + n[0].length) : -1;
  }
  function parseMonth(d, string, i) {
    var n = monthRe.exec(string.slice(i));
    return n ? (d.m = monthLookup.get(n[0].toLowerCase()), i + n[0].length) : -1;
  }
  function parseLocaleDateTime(d, string, i) {
    return parseSpecifier(d, locale_dateTime, string, i);
  }
  function parseLocaleDate(d, string, i) {
    return parseSpecifier(d, locale_date, string, i);
  }
  function parseLocaleTime(d, string, i) {
    return parseSpecifier(d, locale_time, string, i);
  }
  function formatShortWeekday(d) {
    return locale_shortWeekdays[d.getDay()];
  }
  function formatWeekday(d) {
    return locale_weekdays[d.getDay()];
  }
  function formatShortMonth(d) {
    return locale_shortMonths[d.getMonth()];
  }
  function formatMonth(d) {
    return locale_months[d.getMonth()];
  }
  function formatPeriod(d) {
    return locale_periods[+(d.getHours() >= 12)];
  }
  function formatQuarter(d) {
    return 1 + ~~(d.getMonth() / 3);
  }
  function formatUTCShortWeekday(d) {
    return locale_shortWeekdays[d.getUTCDay()];
  }
  function formatUTCWeekday(d) {
    return locale_weekdays[d.getUTCDay()];
  }
  function formatUTCShortMonth(d) {
    return locale_shortMonths[d.getUTCMonth()];
  }
  function formatUTCMonth(d) {
    return locale_months[d.getUTCMonth()];
  }
  function formatUTCPeriod(d) {
    return locale_periods[+(d.getUTCHours() >= 12)];
  }
  function formatUTCQuarter(d) {
    return 1 + ~~(d.getUTCMonth() / 3);
  }
  return {
    format: function(specifier) {
      var f = newFormat(specifier += "", formats);
      f.toString = function() {
        return specifier;
      };
      return f;
    },
    parse: function(specifier) {
      var p = newParse(specifier += "", false);
      p.toString = function() {
        return specifier;
      };
      return p;
    },
    utcFormat: function(specifier) {
      var f = newFormat(specifier += "", utcFormats);
      f.toString = function() {
        return specifier;
      };
      return f;
    },
    utcParse: function(specifier) {
      var p = newParse(specifier += "", true);
      p.toString = function() {
        return specifier;
      };
      return p;
    }
  };
}
var pads = { "-": "", "_": " ", "0": "0" };
var numberRe = /^\s*\d+/;
var percentRe = /^%/;
var requoteRe = /[\\^$*+?|[\]().{}]/g;
function pad(value, fill, width) {
  var sign = value < 0 ? "-" : "", string = (sign ? -value : value) + "", length = string.length;
  return sign + (length < width ? new Array(width - length + 1).join(fill) + string : string);
}
function requote(s) {
  return s.replace(requoteRe, "\\$&");
}
function formatRe(names) {
  return new RegExp("^(?:" + names.map(requote).join("|") + ")", "i");
}
function formatLookup(names) {
  return new Map(names.map((name, i) => [name.toLowerCase(), i]));
}
function parseWeekdayNumberSunday(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 1));
  return n ? (d.w = +n[0], i + n[0].length) : -1;
}
function parseWeekdayNumberMonday(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 1));
  return n ? (d.u = +n[0], i + n[0].length) : -1;
}
function parseWeekNumberSunday(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 2));
  return n ? (d.U = +n[0], i + n[0].length) : -1;
}
function parseWeekNumberISO(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 2));
  return n ? (d.V = +n[0], i + n[0].length) : -1;
}
function parseWeekNumberMonday(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 2));
  return n ? (d.W = +n[0], i + n[0].length) : -1;
}
function parseFullYear(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 4));
  return n ? (d.y = +n[0], i + n[0].length) : -1;
}
function parseYear(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 2));
  return n ? (d.y = +n[0] + (+n[0] > 68 ? 1900 : 2e3), i + n[0].length) : -1;
}
function parseZone(d, string, i) {
  var n = /^(Z)|([+-]\d\d)(?::?(\d\d))?/.exec(string.slice(i, i + 6));
  return n ? (d.Z = n[1] ? 0 : -(n[2] + (n[3] || "00")), i + n[0].length) : -1;
}
function parseQuarter(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 1));
  return n ? (d.q = n[0] * 3 - 3, i + n[0].length) : -1;
}
function parseMonthNumber(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 2));
  return n ? (d.m = n[0] - 1, i + n[0].length) : -1;
}
function parseDayOfMonth(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 2));
  return n ? (d.d = +n[0], i + n[0].length) : -1;
}
function parseDayOfYear(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 3));
  return n ? (d.m = 0, d.d = +n[0], i + n[0].length) : -1;
}
function parseHour24(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 2));
  return n ? (d.H = +n[0], i + n[0].length) : -1;
}
function parseMinutes(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 2));
  return n ? (d.M = +n[0], i + n[0].length) : -1;
}
function parseSeconds(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 2));
  return n ? (d.S = +n[0], i + n[0].length) : -1;
}
function parseMilliseconds(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 3));
  return n ? (d.L = +n[0], i + n[0].length) : -1;
}
function parseMicroseconds(d, string, i) {
  var n = numberRe.exec(string.slice(i, i + 6));
  return n ? (d.L = Math.floor(n[0] / 1e3), i + n[0].length) : -1;
}
function parseLiteralPercent(d, string, i) {
  var n = percentRe.exec(string.slice(i, i + 1));
  return n ? i + n[0].length : -1;
}
function parseUnixTimestamp(d, string, i) {
  var n = numberRe.exec(string.slice(i));
  return n ? (d.Q = +n[0], i + n[0].length) : -1;
}
function parseUnixTimestampSeconds(d, string, i) {
  var n = numberRe.exec(string.slice(i));
  return n ? (d.s = +n[0], i + n[0].length) : -1;
}
function formatDayOfMonth(d, p) {
  return pad(d.getDate(), p, 2);
}
function formatHour24(d, p) {
  return pad(d.getHours(), p, 2);
}
function formatHour12(d, p) {
  return pad(d.getHours() % 12 || 12, p, 2);
}
function formatDayOfYear(d, p) {
  return pad(1 + timeDay.count(timeYear(d), d), p, 3);
}
function formatMilliseconds(d, p) {
  return pad(d.getMilliseconds(), p, 3);
}
function formatMicroseconds(d, p) {
  return formatMilliseconds(d, p) + "000";
}
function formatMonthNumber(d, p) {
  return pad(d.getMonth() + 1, p, 2);
}
function formatMinutes(d, p) {
  return pad(d.getMinutes(), p, 2);
}
function formatSeconds(d, p) {
  return pad(d.getSeconds(), p, 2);
}
function formatWeekdayNumberMonday(d) {
  var day = d.getDay();
  return day === 0 ? 7 : day;
}
function formatWeekNumberSunday(d, p) {
  return pad(timeSunday.count(timeYear(d) - 1, d), p, 2);
}
function dISO(d) {
  var day = d.getDay();
  return day >= 4 || day === 0 ? timeThursday(d) : timeThursday.ceil(d);
}
function formatWeekNumberISO(d, p) {
  d = dISO(d);
  return pad(timeThursday.count(timeYear(d), d) + (timeYear(d).getDay() === 4), p, 2);
}
function formatWeekdayNumberSunday(d) {
  return d.getDay();
}
function formatWeekNumberMonday(d, p) {
  return pad(timeMonday.count(timeYear(d) - 1, d), p, 2);
}
function formatYear(d, p) {
  return pad(d.getFullYear() % 100, p, 2);
}
function formatYearISO(d, p) {
  d = dISO(d);
  return pad(d.getFullYear() % 100, p, 2);
}
function formatFullYear(d, p) {
  return pad(d.getFullYear() % 1e4, p, 4);
}
function formatFullYearISO(d, p) {
  var day = d.getDay();
  d = day >= 4 || day === 0 ? timeThursday(d) : timeThursday.ceil(d);
  return pad(d.getFullYear() % 1e4, p, 4);
}
function formatZone(d) {
  var z = d.getTimezoneOffset();
  return (z > 0 ? "-" : (z *= -1, "+")) + pad(z / 60 | 0, "0", 2) + pad(z % 60, "0", 2);
}
function formatUTCDayOfMonth(d, p) {
  return pad(d.getUTCDate(), p, 2);
}
function formatUTCHour24(d, p) {
  return pad(d.getUTCHours(), p, 2);
}
function formatUTCHour12(d, p) {
  return pad(d.getUTCHours() % 12 || 12, p, 2);
}
function formatUTCDayOfYear(d, p) {
  return pad(1 + utcDay.count(utcYear(d), d), p, 3);
}
function formatUTCMilliseconds(d, p) {
  return pad(d.getUTCMilliseconds(), p, 3);
}
function formatUTCMicroseconds(d, p) {
  return formatUTCMilliseconds(d, p) + "000";
}
function formatUTCMonthNumber(d, p) {
  return pad(d.getUTCMonth() + 1, p, 2);
}
function formatUTCMinutes(d, p) {
  return pad(d.getUTCMinutes(), p, 2);
}
function formatUTCSeconds(d, p) {
  return pad(d.getUTCSeconds(), p, 2);
}
function formatUTCWeekdayNumberMonday(d) {
  var dow = d.getUTCDay();
  return dow === 0 ? 7 : dow;
}
function formatUTCWeekNumberSunday(d, p) {
  return pad(utcSunday.count(utcYear(d) - 1, d), p, 2);
}
function UTCdISO(d) {
  var day = d.getUTCDay();
  return day >= 4 || day === 0 ? utcThursday(d) : utcThursday.ceil(d);
}
function formatUTCWeekNumberISO(d, p) {
  d = UTCdISO(d);
  return pad(utcThursday.count(utcYear(d), d) + (utcYear(d).getUTCDay() === 4), p, 2);
}
function formatUTCWeekdayNumberSunday(d) {
  return d.getUTCDay();
}
function formatUTCWeekNumberMonday(d, p) {
  return pad(utcMonday.count(utcYear(d) - 1, d), p, 2);
}
function formatUTCYear(d, p) {
  return pad(d.getUTCFullYear() % 100, p, 2);
}
function formatUTCYearISO(d, p) {
  d = UTCdISO(d);
  return pad(d.getUTCFullYear() % 100, p, 2);
}
function formatUTCFullYear(d, p) {
  return pad(d.getUTCFullYear() % 1e4, p, 4);
}
function formatUTCFullYearISO(d, p) {
  var day = d.getUTCDay();
  d = day >= 4 || day === 0 ? utcThursday(d) : utcThursday.ceil(d);
  return pad(d.getUTCFullYear() % 1e4, p, 4);
}
function formatUTCZone() {
  return "+0000";
}
function formatLiteralPercent() {
  return "%";
}
function formatUnixTimestamp(d) {
  return +d;
}
function formatUnixTimestampSeconds(d) {
  return Math.floor(+d / 1e3);
}

// node_modules/d3-time-format/src/defaultLocale.js
var locale2;
var timeFormat;
var timeParse;
var utcFormat;
var utcParse;
defaultLocale2({
  dateTime: "%x, %X",
  date: "%-m/%-d/%Y",
  time: "%-I:%M:%S %p",
  periods: ["AM", "PM"],
  days: ["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"],
  shortDays: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
  months: ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"],
  shortMonths: ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]
});
function defaultLocale2(definition) {
  locale2 = formatLocale(definition);
  timeFormat = locale2.format;
  timeParse = locale2.parse;
  utcFormat = locale2.utcFormat;
  utcParse = locale2.utcParse;
  return locale2;
}

// node_modules/d3-scale/src/time.js
function date(t) {
  return new Date(t);
}
function number4(t) {
  return t instanceof Date ? +t : +/* @__PURE__ */ new Date(+t);
}
function calendar(ticks2, tickInterval, year, month, week, day, hour, minute, second2, format2) {
  var scale = continuous(), invert = scale.invert, domain = scale.domain;
  var formatMillisecond = format2(".%L"), formatSecond = format2(":%S"), formatMinute = format2("%I:%M"), formatHour = format2("%I %p"), formatDay = format2("%a %d"), formatWeek = format2("%b %d"), formatMonth = format2("%B"), formatYear2 = format2("%Y");
  function tickFormat2(date2) {
    return (second2(date2) < date2 ? formatMillisecond : minute(date2) < date2 ? formatSecond : hour(date2) < date2 ? formatMinute : day(date2) < date2 ? formatHour : month(date2) < date2 ? week(date2) < date2 ? formatDay : formatWeek : year(date2) < date2 ? formatMonth : formatYear2)(date2);
  }
  scale.invert = function(y2) {
    return new Date(invert(y2));
  };
  scale.domain = function(_) {
    return arguments.length ? domain(Array.from(_, number4)) : domain().map(date);
  };
  scale.ticks = function(interval2) {
    var d = domain();
    return ticks2(d[0], d[d.length - 1], interval2 == null ? 10 : interval2);
  };
  scale.tickFormat = function(count, specifier) {
    return specifier == null ? tickFormat2 : format2(specifier);
  };
  scale.nice = function(interval2) {
    var d = domain();
    if (!interval2 || typeof interval2.range !== "function") interval2 = tickInterval(d[0], d[d.length - 1], interval2 == null ? 10 : interval2);
    return interval2 ? domain(nice(d, interval2)) : scale;
  };
  scale.copy = function() {
    return copy(scale, calendar(ticks2, tickInterval, year, month, week, day, hour, minute, second2, format2));
  };
  return scale;
}

// node_modules/d3-scale/src/utcTime.js
function utcTime() {
  return initRange.apply(calendar(utcTicks, utcTickInterval, utcYear, utcMonth, utcSunday, utcDay, utcHour, utcMinute, second, utcFormat).domain([Date.UTC(2e3, 0, 1), Date.UTC(2e3, 0, 2)]), arguments);
}

// node_modules/d3-shape/src/constant.js
function constant_default4(x2) {
  return function constant() {
    return x2;
  };
}

// node_modules/d3-shape/src/path.js
function withPath(shape) {
  let digits = 3;
  shape.digits = function(_) {
    if (!arguments.length) return digits;
    if (_ == null) {
      digits = null;
    } else {
      const d = Math.floor(_);
      if (!(d >= 0)) throw new RangeError(`invalid digits: ${_}`);
      digits = d;
    }
    return shape;
  };
  return () => new Path(digits);
}

// node_modules/d3-shape/src/array.js
var slice = Array.prototype.slice;
function array_default(x2) {
  return typeof x2 === "object" && "length" in x2 ? x2 : Array.from(x2);
}

// node_modules/d3-shape/src/curve/linear.js
function Linear(context) {
  this._context = context;
}
Linear.prototype = {
  areaStart: function() {
    this._line = 0;
  },
  areaEnd: function() {
    this._line = NaN;
  },
  lineStart: function() {
    this._point = 0;
  },
  lineEnd: function() {
    if (this._line || this._line !== 0 && this._point === 1) this._context.closePath();
    this._line = 1 - this._line;
  },
  point: function(x2, y2) {
    x2 = +x2, y2 = +y2;
    switch (this._point) {
      case 0:
        this._point = 1;
        this._line ? this._context.lineTo(x2, y2) : this._context.moveTo(x2, y2);
        break;
      case 1:
        this._point = 2;
      // falls through
      default:
        this._context.lineTo(x2, y2);
        break;
    }
  }
};
function linear_default(context) {
  return new Linear(context);
}

// node_modules/d3-shape/src/point.js
function x(p) {
  return p[0];
}
function y(p) {
  return p[1];
}

// node_modules/d3-shape/src/line.js
function line_default(x2, y2) {
  var defined = constant_default4(true), context = null, curve = linear_default, output = null, path2 = withPath(line);
  x2 = typeof x2 === "function" ? x2 : x2 === void 0 ? x : constant_default4(x2);
  y2 = typeof y2 === "function" ? y2 : y2 === void 0 ? y : constant_default4(y2);
  function line(data) {
    var i, n = (data = array_default(data)).length, d, defined0 = false, buffer;
    if (context == null) output = curve(buffer = path2());
    for (i = 0; i <= n; ++i) {
      if (!(i < n && defined(d = data[i], i, data)) === defined0) {
        if (defined0 = !defined0) output.lineStart();
        else output.lineEnd();
      }
      if (defined0) output.point(+x2(d, i, data), +y2(d, i, data));
    }
    if (buffer) return output = null, buffer + "" || null;
  }
  line.x = function(_) {
    return arguments.length ? (x2 = typeof _ === "function" ? _ : constant_default4(+_), line) : x2;
  };
  line.y = function(_) {
    return arguments.length ? (y2 = typeof _ === "function" ? _ : constant_default4(+_), line) : y2;
  };
  line.defined = function(_) {
    return arguments.length ? (defined = typeof _ === "function" ? _ : constant_default4(!!_), line) : defined;
  };
  line.curve = function(_) {
    return arguments.length ? (curve = _, context != null && (output = curve(context)), line) : curve;
  };
  line.context = function(_) {
    return arguments.length ? (_ == null ? context = output = null : output = curve(context = _), line) : context;
  };
  return line;
}

// node_modules/d3-shape/src/curve/step.js
function Step(context, t) {
  this._context = context;
  this._t = t;
}
Step.prototype = {
  areaStart: function() {
    this._line = 0;
  },
  areaEnd: function() {
    this._line = NaN;
  },
  lineStart: function() {
    this._x = this._y = NaN;
    this._point = 0;
  },
  lineEnd: function() {
    if (0 < this._t && this._t < 1 && this._point === 2) this._context.lineTo(this._x, this._y);
    if (this._line || this._line !== 0 && this._point === 1) this._context.closePath();
    if (this._line >= 0) this._t = 1 - this._t, this._line = 1 - this._line;
  },
  point: function(x2, y2) {
    x2 = +x2, y2 = +y2;
    switch (this._point) {
      case 0:
        this._point = 1;
        this._line ? this._context.lineTo(x2, y2) : this._context.moveTo(x2, y2);
        break;
      case 1:
        this._point = 2;
      // falls through
      default: {
        if (this._t <= 0) {
          this._context.lineTo(this._x, y2);
          this._context.lineTo(x2, y2);
        } else {
          var x1 = this._x * (1 - this._t) + x2 * this._t;
          this._context.lineTo(x1, this._y);
          this._context.lineTo(x1, y2);
        }
        break;
      }
    }
    this._x = x2, this._y = y2;
  }
};
function stepAfter(context) {
  return new Step(context, 1);
}

// node_modules/d3-zoom/src/constant.js
var constant_default5 = (x2) => () => x2;

// node_modules/d3-zoom/src/event.js
function ZoomEvent(type2, {
  sourceEvent,
  target,
  transform: transform2,
  dispatch: dispatch2
}) {
  Object.defineProperties(this, {
    type: { value: type2, enumerable: true, configurable: true },
    sourceEvent: { value: sourceEvent, enumerable: true, configurable: true },
    target: { value: target, enumerable: true, configurable: true },
    transform: { value: transform2, enumerable: true, configurable: true },
    _: { value: dispatch2 }
  });
}

// node_modules/d3-zoom/src/transform.js
function Transform(k, x2, y2) {
  this.k = k;
  this.x = x2;
  this.y = y2;
}
Transform.prototype = {
  constructor: Transform,
  scale: function(k) {
    return k === 1 ? this : new Transform(this.k * k, this.x, this.y);
  },
  translate: function(x2, y2) {
    return x2 === 0 & y2 === 0 ? this : new Transform(this.k, this.x + this.k * x2, this.y + this.k * y2);
  },
  apply: function(point) {
    return [point[0] * this.k + this.x, point[1] * this.k + this.y];
  },
  applyX: function(x2) {
    return x2 * this.k + this.x;
  },
  applyY: function(y2) {
    return y2 * this.k + this.y;
  },
  invert: function(location) {
    return [(location[0] - this.x) / this.k, (location[1] - this.y) / this.k];
  },
  invertX: function(x2) {
    return (x2 - this.x) / this.k;
  },
  invertY: function(y2) {
    return (y2 - this.y) / this.k;
  },
  rescaleX: function(x2) {
    return x2.copy().domain(x2.range().map(this.invertX, this).map(x2.invert, x2));
  },
  rescaleY: function(y2) {
    return y2.copy().domain(y2.range().map(this.invertY, this).map(y2.invert, y2));
  },
  toString: function() {
    return "translate(" + this.x + "," + this.y + ") scale(" + this.k + ")";
  }
};
var identity3 = new Transform(1, 0, 0);
transform.prototype = Transform.prototype;
function transform(node) {
  while (!node.__zoom) if (!(node = node.parentNode)) return identity3;
  return node.__zoom;
}

// node_modules/d3-zoom/src/noevent.js
function nopropagation2(event) {
  event.stopImmediatePropagation();
}
function noevent_default3(event) {
  event.preventDefault();
  event.stopImmediatePropagation();
}

// node_modules/d3-zoom/src/zoom.js
function defaultFilter(event) {
  return (!event.ctrlKey || event.type === "wheel") && !event.button;
}
function defaultExtent() {
  var e = this;
  if (e instanceof SVGElement) {
    e = e.ownerSVGElement || e;
    if (e.hasAttribute("viewBox")) {
      e = e.viewBox.baseVal;
      return [[e.x, e.y], [e.x + e.width, e.y + e.height]];
    }
    return [[0, 0], [e.width.baseVal.value, e.height.baseVal.value]];
  }
  return [[0, 0], [e.clientWidth, e.clientHeight]];
}
function defaultTransform() {
  return this.__zoom || identity3;
}
function defaultWheelDelta(event) {
  return -event.deltaY * (event.deltaMode === 1 ? 0.05 : event.deltaMode ? 1 : 2e-3) * (event.ctrlKey ? 10 : 1);
}
function defaultTouchable() {
  return navigator.maxTouchPoints || "ontouchstart" in this;
}
function defaultConstrain(transform2, extent2, translateExtent) {
  var dx0 = transform2.invertX(extent2[0][0]) - translateExtent[0][0], dx1 = transform2.invertX(extent2[1][0]) - translateExtent[1][0], dy0 = transform2.invertY(extent2[0][1]) - translateExtent[0][1], dy1 = transform2.invertY(extent2[1][1]) - translateExtent[1][1];
  return transform2.translate(
    dx1 > dx0 ? (dx0 + dx1) / 2 : Math.min(0, dx0) || Math.max(0, dx1),
    dy1 > dy0 ? (dy0 + dy1) / 2 : Math.min(0, dy0) || Math.max(0, dy1)
  );
}
function zoom_default2() {
  var filter2 = defaultFilter, extent2 = defaultExtent, constrain = defaultConstrain, wheelDelta = defaultWheelDelta, touchable = defaultTouchable, scaleExtent = [0, Infinity], translateExtent = [[-Infinity, -Infinity], [Infinity, Infinity]], duration = 250, interpolate = zoom_default, listeners = dispatch_default("start", "zoom", "end"), touchstarting, touchfirst, touchending, touchDelay = 500, wheelDelay = 150, clickDistance2 = 0, tapDistance = 10;
  function zoom(selection2) {
    selection2.property("__zoom", defaultTransform).on("wheel.zoom", wheeled, { passive: false }).on("mousedown.zoom", mousedowned).on("dblclick.zoom", dblclicked).filter(touchable).on("touchstart.zoom", touchstarted).on("touchmove.zoom", touchmoved).on("touchend.zoom touchcancel.zoom", touchended).style("-webkit-tap-highlight-color", "rgba(0,0,0,0)");
  }
  zoom.transform = function(collection, transform2, point, event) {
    var selection2 = collection.selection ? collection.selection() : collection;
    selection2.property("__zoom", defaultTransform);
    if (collection !== selection2) {
      schedule(collection, transform2, point, event);
    } else {
      selection2.interrupt().each(function() {
        gesture(this, arguments).event(event).start().zoom(null, typeof transform2 === "function" ? transform2.apply(this, arguments) : transform2).end();
      });
    }
  };
  zoom.scaleBy = function(selection2, k, p, event) {
    zoom.scaleTo(selection2, function() {
      var k0 = this.__zoom.k, k1 = typeof k === "function" ? k.apply(this, arguments) : k;
      return k0 * k1;
    }, p, event);
  };
  zoom.scaleTo = function(selection2, k, p, event) {
    zoom.transform(selection2, function() {
      var e = extent2.apply(this, arguments), t02 = this.__zoom, p0 = p == null ? centroid(e) : typeof p === "function" ? p.apply(this, arguments) : p, p1 = t02.invert(p0), k1 = typeof k === "function" ? k.apply(this, arguments) : k;
      return constrain(translate(scale(t02, k1), p0, p1), e, translateExtent);
    }, p, event);
  };
  zoom.translateBy = function(selection2, x2, y2, event) {
    zoom.transform(selection2, function() {
      return constrain(this.__zoom.translate(
        typeof x2 === "function" ? x2.apply(this, arguments) : x2,
        typeof y2 === "function" ? y2.apply(this, arguments) : y2
      ), extent2.apply(this, arguments), translateExtent);
    }, null, event);
  };
  zoom.translateTo = function(selection2, x2, y2, p, event) {
    zoom.transform(selection2, function() {
      var e = extent2.apply(this, arguments), t = this.__zoom, p0 = p == null ? centroid(e) : typeof p === "function" ? p.apply(this, arguments) : p;
      return constrain(identity3.translate(p0[0], p0[1]).scale(t.k).translate(
        typeof x2 === "function" ? -x2.apply(this, arguments) : -x2,
        typeof y2 === "function" ? -y2.apply(this, arguments) : -y2
      ), e, translateExtent);
    }, p, event);
  };
  function scale(transform2, k) {
    k = Math.max(scaleExtent[0], Math.min(scaleExtent[1], k));
    return k === transform2.k ? transform2 : new Transform(k, transform2.x, transform2.y);
  }
  function translate(transform2, p0, p1) {
    var x2 = p0[0] - p1[0] * transform2.k, y2 = p0[1] - p1[1] * transform2.k;
    return x2 === transform2.x && y2 === transform2.y ? transform2 : new Transform(transform2.k, x2, y2);
  }
  function centroid(extent3) {
    return [(+extent3[0][0] + +extent3[1][0]) / 2, (+extent3[0][1] + +extent3[1][1]) / 2];
  }
  function schedule(transition2, transform2, point, event) {
    transition2.on("start.zoom", function() {
      gesture(this, arguments).event(event).start();
    }).on("interrupt.zoom end.zoom", function() {
      gesture(this, arguments).event(event).end();
    }).tween("zoom", function() {
      var that = this, args = arguments, g = gesture(that, args).event(event), e = extent2.apply(that, args), p = point == null ? centroid(e) : typeof point === "function" ? point.apply(that, args) : point, w = Math.max(e[1][0] - e[0][0], e[1][1] - e[0][1]), a = that.__zoom, b = typeof transform2 === "function" ? transform2.apply(that, args) : transform2, i = interpolate(a.invert(p).concat(w / a.k), b.invert(p).concat(w / b.k));
      return function(t) {
        if (t === 1) t = b;
        else {
          var l = i(t), k = w / l[2];
          t = new Transform(k, p[0] - l[0] * k, p[1] - l[1] * k);
        }
        g.zoom(null, t);
      };
    });
  }
  function gesture(that, args, clean) {
    return !clean && that.__zooming || new Gesture(that, args);
  }
  function Gesture(that, args) {
    this.that = that;
    this.args = args;
    this.active = 0;
    this.sourceEvent = null;
    this.extent = extent2.apply(that, args);
    this.taps = 0;
  }
  Gesture.prototype = {
    event: function(event) {
      if (event) this.sourceEvent = event;
      return this;
    },
    start: function() {
      if (++this.active === 1) {
        this.that.__zooming = this;
        this.emit("start");
      }
      return this;
    },
    zoom: function(key, transform2) {
      if (this.mouse && key !== "mouse") this.mouse[1] = transform2.invert(this.mouse[0]);
      if (this.touch0 && key !== "touch") this.touch0[1] = transform2.invert(this.touch0[0]);
      if (this.touch1 && key !== "touch") this.touch1[1] = transform2.invert(this.touch1[0]);
      this.that.__zoom = transform2;
      this.emit("zoom");
      return this;
    },
    end: function() {
      if (--this.active === 0) {
        delete this.that.__zooming;
        this.emit("end");
      }
      return this;
    },
    emit: function(type2) {
      var d = select_default2(this.that).datum();
      listeners.call(
        type2,
        this.that,
        new ZoomEvent(type2, {
          sourceEvent: this.sourceEvent,
          target: zoom,
          type: type2,
          transform: this.that.__zoom,
          dispatch: listeners
        }),
        d
      );
    }
  };
  function wheeled(event, ...args) {
    if (!filter2.apply(this, arguments)) return;
    var g = gesture(this, args).event(event), t = this.__zoom, k = Math.max(scaleExtent[0], Math.min(scaleExtent[1], t.k * Math.pow(2, wheelDelta.apply(this, arguments)))), p = pointer_default(event);
    if (g.wheel) {
      if (g.mouse[0][0] !== p[0] || g.mouse[0][1] !== p[1]) {
        g.mouse[1] = t.invert(g.mouse[0] = p);
      }
      clearTimeout(g.wheel);
    } else if (t.k === k) return;
    else {
      g.mouse = [p, t.invert(p)];
      interrupt_default(this);
      g.start();
    }
    noevent_default3(event);
    g.wheel = setTimeout(wheelidled, wheelDelay);
    g.zoom("mouse", constrain(translate(scale(t, k), g.mouse[0], g.mouse[1]), g.extent, translateExtent));
    function wheelidled() {
      g.wheel = null;
      g.end();
    }
  }
  function mousedowned(event, ...args) {
    if (touchending || !filter2.apply(this, arguments)) return;
    var currentTarget = event.currentTarget, g = gesture(this, args, true).event(event), v = select_default2(event.view).on("mousemove.zoom", mousemoved, true).on("mouseup.zoom", mouseupped, true), p = pointer_default(event, currentTarget), x0 = event.clientX, y0 = event.clientY;
    nodrag_default(event.view);
    nopropagation2(event);
    g.mouse = [p, this.__zoom.invert(p)];
    interrupt_default(this);
    g.start();
    function mousemoved(event2) {
      noevent_default3(event2);
      if (!g.moved) {
        var dx = event2.clientX - x0, dy = event2.clientY - y0;
        g.moved = dx * dx + dy * dy > clickDistance2;
      }
      g.event(event2).zoom("mouse", constrain(translate(g.that.__zoom, g.mouse[0] = pointer_default(event2, currentTarget), g.mouse[1]), g.extent, translateExtent));
    }
    function mouseupped(event2) {
      v.on("mousemove.zoom mouseup.zoom", null);
      yesdrag(event2.view, g.moved);
      noevent_default3(event2);
      g.event(event2).end();
    }
  }
  function dblclicked(event, ...args) {
    if (!filter2.apply(this, arguments)) return;
    var t02 = this.__zoom, p0 = pointer_default(event.changedTouches ? event.changedTouches[0] : event, this), p1 = t02.invert(p0), k1 = t02.k * (event.shiftKey ? 0.5 : 2), t12 = constrain(translate(scale(t02, k1), p0, p1), extent2.apply(this, args), translateExtent);
    noevent_default3(event);
    if (duration > 0) select_default2(this).transition().duration(duration).call(schedule, t12, p0, event);
    else select_default2(this).call(zoom.transform, t12, p0, event);
  }
  function touchstarted(event, ...args) {
    if (!filter2.apply(this, arguments)) return;
    var touches = event.touches, n = touches.length, g = gesture(this, args, event.changedTouches.length === n).event(event), started, i, t, p;
    nopropagation2(event);
    for (i = 0; i < n; ++i) {
      t = touches[i], p = pointer_default(t, this);
      p = [p, this.__zoom.invert(p), t.identifier];
      if (!g.touch0) g.touch0 = p, started = true, g.taps = 1 + !!touchstarting;
      else if (!g.touch1 && g.touch0[2] !== p[2]) g.touch1 = p, g.taps = 0;
    }
    if (touchstarting) touchstarting = clearTimeout(touchstarting);
    if (started) {
      if (g.taps < 2) touchfirst = p[0], touchstarting = setTimeout(function() {
        touchstarting = null;
      }, touchDelay);
      interrupt_default(this);
      g.start();
    }
  }
  function touchmoved(event, ...args) {
    if (!this.__zooming) return;
    var g = gesture(this, args).event(event), touches = event.changedTouches, n = touches.length, i, t, p, l;
    noevent_default3(event);
    for (i = 0; i < n; ++i) {
      t = touches[i], p = pointer_default(t, this);
      if (g.touch0 && g.touch0[2] === t.identifier) g.touch0[0] = p;
      else if (g.touch1 && g.touch1[2] === t.identifier) g.touch1[0] = p;
    }
    t = g.that.__zoom;
    if (g.touch1) {
      var p0 = g.touch0[0], l0 = g.touch0[1], p1 = g.touch1[0], l1 = g.touch1[1], dp = (dp = p1[0] - p0[0]) * dp + (dp = p1[1] - p0[1]) * dp, dl = (dl = l1[0] - l0[0]) * dl + (dl = l1[1] - l0[1]) * dl;
      t = scale(t, Math.sqrt(dp / dl));
      p = [(p0[0] + p1[0]) / 2, (p0[1] + p1[1]) / 2];
      l = [(l0[0] + l1[0]) / 2, (l0[1] + l1[1]) / 2];
    } else if (g.touch0) p = g.touch0[0], l = g.touch0[1];
    else return;
    g.zoom("touch", constrain(translate(t, p, l), g.extent, translateExtent));
  }
  function touchended(event, ...args) {
    if (!this.__zooming) return;
    var g = gesture(this, args).event(event), touches = event.changedTouches, n = touches.length, i, t;
    nopropagation2(event);
    if (touchending) clearTimeout(touchending);
    touchending = setTimeout(function() {
      touchending = null;
    }, touchDelay);
    for (i = 0; i < n; ++i) {
      t = touches[i];
      if (g.touch0 && g.touch0[2] === t.identifier) delete g.touch0;
      else if (g.touch1 && g.touch1[2] === t.identifier) delete g.touch1;
    }
    if (g.touch1 && !g.touch0) g.touch0 = g.touch1, delete g.touch1;
    if (g.touch0) g.touch0[1] = this.__zoom.invert(g.touch0[0]);
    else {
      g.end();
      if (g.taps === 2) {
        t = pointer_default(t, this);
        if (Math.hypot(touchfirst[0] - t[0], touchfirst[1] - t[1]) < tapDistance) {
          var p = select_default2(this).on("dblclick.zoom");
          if (p) p.apply(this, arguments);
        }
      }
    }
  }
  zoom.wheelDelta = function(_) {
    return arguments.length ? (wheelDelta = typeof _ === "function" ? _ : constant_default5(+_), zoom) : wheelDelta;
  };
  zoom.filter = function(_) {
    return arguments.length ? (filter2 = typeof _ === "function" ? _ : constant_default5(!!_), zoom) : filter2;
  };
  zoom.touchable = function(_) {
    return arguments.length ? (touchable = typeof _ === "function" ? _ : constant_default5(!!_), zoom) : touchable;
  };
  zoom.extent = function(_) {
    return arguments.length ? (extent2 = typeof _ === "function" ? _ : constant_default5([[+_[0][0], +_[0][1]], [+_[1][0], +_[1][1]]]), zoom) : extent2;
  };
  zoom.scaleExtent = function(_) {
    return arguments.length ? (scaleExtent[0] = +_[0], scaleExtent[1] = +_[1], zoom) : [scaleExtent[0], scaleExtent[1]];
  };
  zoom.translateExtent = function(_) {
    return arguments.length ? (translateExtent[0][0] = +_[0][0], translateExtent[1][0] = +_[1][0], translateExtent[0][1] = +_[0][1], translateExtent[1][1] = +_[1][1], zoom) : [[translateExtent[0][0], translateExtent[0][1]], [translateExtent[1][0], translateExtent[1][1]]];
  };
  zoom.constrain = function(_) {
    return arguments.length ? (constrain = _, zoom) : constrain;
  };
  zoom.duration = function(_) {
    return arguments.length ? (duration = +_, zoom) : duration;
  };
  zoom.interpolate = function(_) {
    return arguments.length ? (interpolate = _, zoom) : interpolate;
  };
  zoom.on = function() {
    var value = listeners.on.apply(listeners, arguments);
    return value === listeners ? zoom : value;
  };
  zoom.clickDistance = function(_) {
    return arguments.length ? (clickDistance2 = (_ = +_) * _, zoom) : Math.sqrt(clickDistance2);
  };
  zoom.tapDistance = function(_) {
    return arguments.length ? (tapDistance = +_, zoom) : tapDistance;
  };
  return zoom;
}

// graph.ts
function prettyBytes(bytes) {
  const abs2 = Math.abs(bytes);
  if (abs2 > 1024 * 1024) {
    return `${(bytes / 1024 / 1024).toFixed(2)}mb`;
  }
  if (abs2 > 1024) {
    return `${(bytes / 1024).toFixed(0)}kb`;
  }
  return `${bytes}b`;
}
function main() {
  let dbCommits = db_default;
  dbCommits.reverse();
  let last = 0;
  const commits = dbCommits.map((c) => {
    const size = c.data?.size ?? last;
    const delta = size - last;
    if (Math.abs(delta) < 2 * 1024) {
      return null;
    }
    last = size;
    return {
      date: new Date(c.date * 1e3),
      desc: c.desc,
      size,
      delta
    };
  }).filter((d) => d != null);
  for (let i = 0; i < commits.length; i++) {
    if (commits[i].size > 0) {
      commits.splice(0, i);
      break;
    }
  }
  const width = 640;
  const height = 400;
  const margin = { top: 20, right: 20, bottom: 30, left: 80 };
  const dateExtent = extent(commits, (d) => d.date);
  const x2 = utcTime().domain([timeDay.offset(dateExtent[0], -1), dateExtent[1]]).range([margin.left, width - margin.right]);
  const y2 = linear2().domain(extent(commits, (d) => d.size)).range([height - margin.bottom, margin.top]).nice();
  const svg = create_default("svg").attr("width", width).attr("height", height);
  const gx = svg.append("g").attr("transform", `translate(0,${height - margin.bottom})`);
  svg.append("g").attr("transform", `translate(${margin.left},0)`).call(axisLeft(y2).tickFormat((b) => prettyBytes(b)).ticks(5));
  svg.append("clipPath").attr("id", "clip").append("rect").attr("x", margin.left).attr("y", margin.top).attr("width", width - margin.left).attr("height", height - margin.top - margin.bottom);
  const clipped = svg.append("g").attr("clip-path", "url(#clip)");
  const path2 = clipped.append("path").attr("fill", "none").attr("stroke", "steelblue").attr("stroke-width", 1.5);
  const dots = clipped.append("g").attr("fill", "white").attr("stroke", "steelblue").attr("stroke-width", 1).selectAll("circle").data(commits.filter((d) => d.size != null)).join("circle").attr("r", 4);
  const tooltip = select_default2("body").append("div").attr("class", "tooltip").style("opacity", 0);
  dots.on("mouseover", function(event, d) {
    select_default2(this).attr("stroke-width", 2);
    tooltip.style("opacity", 1);
    tooltip.text(`${d.desc} (${d.delta >= 0 ? "+" : ""}${prettyBytes(d.delta)})`).style("left", event.pageX + "px").style("top", event.pageY - 28 + "px");
  });
  dots.on("mouseout", function(event, d) {
    select_default2(this).attr("stroke-width", 1);
    tooltip.style("opacity", 0);
  });
  function render(x3) {
    const line = line_default().defined((d) => d.size != null).x((d) => x3(d.date)).y((d) => y2(d.size)).curve(stepAfter);
    path2.attr("d", line(commits));
    dots.attr("transform", (d) => `translate(${x3(d.date)},${y2(d.size)})`);
    gx.call(axisBottom(x3));
  }
  render(x2);
  const zoom = zoom_default2().scaleExtent([1, 200]).extent([[margin.left, 0], [width - margin.right, height]]).translateExtent([[margin.left, -Infinity], [width - margin.right, Infinity]]).on("zoom", zoomed);
  function zoomed(event) {
    const xz = event.transform.rescaleX(x2);
    render(xz);
  }
  svg.call(zoom);
  document.body.appendChild(svg.node());
}
main();
