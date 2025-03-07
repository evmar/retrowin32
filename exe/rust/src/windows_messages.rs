/// Converts a Windows WM_* constant to a string.
pub fn windows_message_to_str(msg: u32) -> Option<&'static str> {
    TAB.get(msg as usize)?.clone()
}

const TAB: [Option<&str>; 0x8001] = {
    let mut tab = [None; 0x8001];
    tab[0x0000] = Some("WM_NULL");
    tab[0x0001] = Some("WM_CREATE");
    tab[0x0002] = Some("WM_DESTROY");
    tab[0x0003] = Some("WM_MOVE");
    tab[0x0005] = Some("WM_SIZE");
    tab[0x0006] = Some("WM_ACTIVATE");
    tab[0x0007] = Some("WM_SETFOCUS");
    tab[0x0008] = Some("WM_KILLFOCUS");
    tab[0x000A] = Some("WM_ENABLE");
    tab[0x000B] = Some("WM_SETREDRAW");
    tab[0x000C] = Some("WM_SETTEXT");
    tab[0x000D] = Some("WM_GETTEXT");
    tab[0x000E] = Some("WM_GETTEXTLENGTH");
    tab[0x000F] = Some("WM_PAINT");
    tab[0x0010] = Some("WM_CLOSE");
    tab[0x0011] = Some("WM_QUERYENDSESSION");
    tab[0x0013] = Some("WM_QUERYOPEN");
    tab[0x0016] = Some("WM_ENDSESSION");
    tab[0x0012] = Some("WM_QUIT");
    tab[0x0014] = Some("WM_ERASEBKGND");
    tab[0x0015] = Some("WM_SYSCOLORCHANGE");
    tab[0x0018] = Some("WM_SHOWWINDOW");
    tab[0x001A] = Some("WM_WININICHANGE");
    tab[0x001B] = Some("WM_DEVMODECHANGE");
    tab[0x001C] = Some("WM_ACTIVATEAPP");
    tab[0x001D] = Some("WM_FONTCHANGE");
    tab[0x001E] = Some("WM_TIMECHANGE");
    tab[0x001F] = Some("WM_CANCELMODE");
    tab[0x0020] = Some("WM_SETCURSOR");
    tab[0x0021] = Some("WM_MOUSEACTIVATE");
    tab[0x0022] = Some("WM_CHILDACTIVATE");
    tab[0x0023] = Some("WM_QUEUESYNC");

    tab[0x0024] = Some("WM_GETMINMAXINFO");
    tab[0x0026] = Some("WM_PAINTICON");
    tab[0x0027] = Some("WM_ICONERASEBKGND");
    tab[0x0028] = Some("WM_NEXTDLGCTL");
    tab[0x002A] = Some("WM_SPOOLERSTATUS");
    tab[0x002B] = Some("WM_DRAWITEM");
    tab[0x002C] = Some("WM_MEASUREITEM");
    tab[0x002D] = Some("WM_DELETEITEM");
    tab[0x002E] = Some("WM_VKEYTOITEM");
    tab[0x002F] = Some("WM_CHARTOITEM");
    tab[0x0030] = Some("WM_SETFONT");
    tab[0x0031] = Some("WM_GETFONT");
    tab[0x0032] = Some("WM_SETHOTKEY");
    tab[0x0033] = Some("WM_GETHOTKEY");
    tab[0x0037] = Some("WM_QUERYDRAGICON");
    tab[0x0039] = Some("WM_COMPAREITEM");

    tab[0x0041] = Some("WM_COMPACTING");
    tab[0x0046] = Some("WM_WINDOWPOSCHANGING");
    tab[0x0047] = Some("WM_WINDOWPOSCHANGED");

    tab[0x0048] = Some("WM_POWER");

    tab[0x004A] = Some("WM_COPYDATA");
    tab[0x004B] = Some("WM_CANCELJOURNAL");

    tab[0x004E] = Some("WM_NOTIFY");
    tab[0x0050] = Some("WM_INPUTLANGCHANGEREQUEST");
    tab[0x0051] = Some("WM_INPUTLANGCHANGE");
    tab[0x0052] = Some("WM_TCARD");
    tab[0x0053] = Some("WM_HELP");
    tab[0x0054] = Some("WM_USERCHANGED");
    tab[0x0055] = Some("WM_NOTIFYFORMAT");

    tab[0x007B] = Some("WM_CONTEXTMENU");
    tab[0x007C] = Some("WM_STYLECHANGING");
    tab[0x007D] = Some("WM_STYLECHANGED");
    tab[0x007E] = Some("WM_DISPLAYCHANGE");
    tab[0x007F] = Some("WM_GETICON");
    tab[0x0080] = Some("WM_SETICON");

    tab[0x0081] = Some("WM_NCCREATE");
    tab[0x0082] = Some("WM_NCDESTROY");
    tab[0x0083] = Some("WM_NCCALCSIZE");
    tab[0x0084] = Some("WM_NCHITTEST");
    tab[0x0085] = Some("WM_NCPAINT");
    tab[0x0086] = Some("WM_NCACTIVATE");
    tab[0x0087] = Some("WM_GETDLGCODE");
    tab[0x0088] = Some("WM_SYNCPAINT");
    tab[0x00A0] = Some("WM_NCMOUSEMOVE");
    tab[0x00A1] = Some("WM_NCLBUTTONDOWN");
    tab[0x00A2] = Some("WM_NCLBUTTONUP");
    tab[0x00A3] = Some("WM_NCLBUTTONDBLCLK");
    tab[0x00A4] = Some("WM_NCRBUTTONDOWN");
    tab[0x00A5] = Some("WM_NCRBUTTONUP");
    tab[0x00A6] = Some("WM_NCRBUTTONDBLCLK");
    tab[0x00A7] = Some("WM_NCMBUTTONDOWN");
    tab[0x00A8] = Some("WM_NCMBUTTONUP");
    tab[0x00A9] = Some("WM_NCMBUTTONDBLCLK");

    tab[0x00AB] = Some("WM_NCXBUTTONDOWN");
    tab[0x00AC] = Some("WM_NCXBUTTONUP");
    tab[0x00AD] = Some("WM_NCXBUTTONDBLCLK");

    tab[0x00FE] = Some("WM_INPUT_DEVICE_CHANGE");

    tab[0x00FF] = Some("WM_INPUT");

    tab[0x0100] = Some("WM_KEYFIRST");
    tab[0x0100] = Some("WM_KEYDOWN");
    tab[0x0101] = Some("WM_KEYUP");
    tab[0x0102] = Some("WM_CHAR");
    tab[0x0103] = Some("WM_DEADCHAR");
    tab[0x0104] = Some("WM_SYSKEYDOWN");
    tab[0x0105] = Some("WM_SYSKEYUP");
    tab[0x0106] = Some("WM_SYSCHAR");
    tab[0x0107] = Some("WM_SYSDEADCHAR");
    tab[0x0109] = Some("WM_UNICHAR");

    tab[0x010D] = Some("WM_IME_STARTCOMPOSITION");
    tab[0x010E] = Some("WM_IME_ENDCOMPOSITION");
    tab[0x010F] = Some("WM_IME_COMPOSITION");
    tab[0x010F] = Some("WM_IME_KEYLAST");
    tab[0x0110] = Some("WM_INITDIALOG");
    tab[0x0111] = Some("WM_COMMAND");
    tab[0x0112] = Some("WM_SYSCOMMAND");
    tab[0x0113] = Some("WM_TIMER");
    tab[0x0114] = Some("WM_HSCROLL");
    tab[0x0115] = Some("WM_VSCROLL");
    tab[0x0116] = Some("WM_INITMENU");
    tab[0x0117] = Some("WM_INITMENUPOPUP");
    tab[0x0119] = Some("WM_GESTURE");
    tab[0x011A] = Some("WM_GESTURENOTIFY");
    tab[0x011F] = Some("WM_MENUSELECT");
    tab[0x0120] = Some("WM_MENUCHAR");
    tab[0x0121] = Some("WM_ENTERIDLE");
    tab[0x0122] = Some("WM_MENURBUTTONUP");
    tab[0x0123] = Some("WM_MENUDRAG");
    tab[0x0124] = Some("WM_MENUGETOBJECT");
    tab[0x0125] = Some("WM_UNINITMENUPOPUP");
    tab[0x0126] = Some("WM_MENUCOMMAND");
    tab[0x0127] = Some("WM_CHANGEUISTATE");
    tab[0x0128] = Some("WM_UPDATEUISTATE");
    tab[0x0129] = Some("WM_QUERYUISTATE");
    tab[0x0132] = Some("WM_CTLCOLORMSGBOX");
    tab[0x0133] = Some("WM_CTLCOLOREDIT");
    tab[0x0134] = Some("WM_CTLCOLORLISTBOX");
    tab[0x0135] = Some("WM_CTLCOLORBTN");
    tab[0x0136] = Some("WM_CTLCOLORDLG");
    tab[0x0137] = Some("WM_CTLCOLORSCROLLBAR");
    tab[0x0138] = Some("WM_CTLCOLORSTATIC");
    tab[0x0200] = Some("WM_MOUSEFIRST");
    tab[0x0200] = Some("WM_MOUSEMOVE");
    tab[0x0201] = Some("WM_LBUTTONDOWN");
    tab[0x0202] = Some("WM_LBUTTONUP");
    tab[0x0203] = Some("WM_LBUTTONDBLCLK");
    tab[0x0204] = Some("WM_RBUTTONDOWN");
    tab[0x0205] = Some("WM_RBUTTONUP");
    tab[0x0206] = Some("WM_RBUTTONDBLCLK");
    tab[0x0207] = Some("WM_MBUTTONDOWN");
    tab[0x0208] = Some("WM_MBUTTONUP");
    tab[0x0209] = Some("WM_MBUTTONDBLCLK");
    tab[0x020A] = Some("WM_MOUSEWHEEL");
    tab[0x020B] = Some("WM_XBUTTONDOWN");
    tab[0x020C] = Some("WM_XBUTTONUP");
    tab[0x020D] = Some("WM_XBUTTONDBLCLK");
    tab[0x020E] = Some("WM_MOUSEHWHEEL");

    tab[0x0210] = Some("WM_PARENTNOTIFY");
    tab[0x0211] = Some("WM_ENTERMENULOOP");
    tab[0x0212] = Some("WM_EXITMENULOOP");

    tab[0x0213] = Some("WM_NEXTMENU");
    tab[0x0214] = Some("WM_SIZING");
    tab[0x0215] = Some("WM_CAPTURECHANGED");
    tab[0x0216] = Some("WM_MOVING");
    tab[0x0218] = Some("WM_POWERBROADCAST");
    tab[0x0219] = Some("WM_DEVICECHANGE");
    tab[0x0220] = Some("WM_MDICREATE");
    tab[0x0221] = Some("WM_MDIDESTROY");
    tab[0x0222] = Some("WM_MDIACTIVATE");
    tab[0x0223] = Some("WM_MDIRESTORE");
    tab[0x0224] = Some("WM_MDINEXT");
    tab[0x0225] = Some("WM_MDIMAXIMIZE");
    tab[0x0226] = Some("WM_MDITILE");
    tab[0x0227] = Some("WM_MDICASCADE");
    tab[0x0228] = Some("WM_MDIICONARRANGE");
    tab[0x0229] = Some("WM_MDIGETACTIVE");
    tab[0x0230] = Some("WM_MDISETMENU");
    tab[0x0231] = Some("WM_ENTERSIZEMOVE");
    tab[0x0232] = Some("WM_EXITSIZEMOVE");
    tab[0x0233] = Some("WM_DROPFILES");
    tab[0x0234] = Some("WM_MDIREFRESHMENU");

    tab[0x238] = Some("WM_POINTERDEVICECHANGE");
    tab[0x239] = Some("WM_POINTERDEVICEINRANGE");
    tab[0x23A] = Some("WM_POINTERDEVICEOUTOFRANGE");

    tab[0x0240] = Some("WM_TOUCH");
    tab[0x0241] = Some("WM_NCPOINTERUPDATE");
    tab[0x0242] = Some("WM_NCPOINTERDOWN");
    tab[0x0243] = Some("WM_NCPOINTERUP");
    tab[0x0245] = Some("WM_POINTERUPDATE");
    tab[0x0246] = Some("WM_POINTERDOWN");
    tab[0x0247] = Some("WM_POINTERUP");
    tab[0x0249] = Some("WM_POINTERENTER");
    tab[0x024A] = Some("WM_POINTERLEAVE");
    tab[0x024B] = Some("WM_POINTERACTIVATE");
    tab[0x024C] = Some("WM_POINTERCAPTURECHANGED");
    tab[0x024D] = Some("WM_TOUCHHITTESTING");
    tab[0x024E] = Some("WM_POINTERWHEEL");
    tab[0x024F] = Some("WM_POINTERHWHEEL");
    tab[0x0250] = Some("DM_POINTERHITTEST");
    tab[0x0251] = Some("WM_POINTERROUTEDTO");
    tab[0x0252] = Some("WM_POINTERROUTEDAWAY");
    tab[0x0253] = Some("WM_POINTERROUTEDRELEASED");
    tab[0x0281] = Some("WM_IME_SETCONTEXT");
    tab[0x0282] = Some("WM_IME_NOTIFY");
    tab[0x0283] = Some("WM_IME_CONTROL");
    tab[0x0284] = Some("WM_IME_COMPOSITIONFULL");
    tab[0x0285] = Some("WM_IME_SELECT");
    tab[0x0286] = Some("WM_IME_CHAR");
    tab[0x0288] = Some("WM_IME_REQUEST");
    tab[0x0290] = Some("WM_IME_KEYDOWN");
    tab[0x0291] = Some("WM_IME_KEYUP");
    tab[0x02A1] = Some("WM_MOUSEHOVER");
    tab[0x02A3] = Some("WM_MOUSELEAVE");

    tab[0x02A0] = Some("WM_NCMOUSEHOVER");
    tab[0x02A2] = Some("WM_NCMOUSELEAVE");
    tab[0x02B1] = Some("WM_WTSSESSION_CHANGE");

    tab[0x0300] = Some("WM_CUT");
    tab[0x0301] = Some("WM_COPY");
    tab[0x0302] = Some("WM_PASTE");
    tab[0x0303] = Some("WM_CLEAR");
    tab[0x0304] = Some("WM_UNDO");
    tab[0x0305] = Some("WM_RENDERFORMAT");
    tab[0x0306] = Some("WM_RENDERALLFORMATS");
    tab[0x0307] = Some("WM_DESTROYCLIPBOARD");
    tab[0x0308] = Some("WM_DRAWCLIPBOARD");
    tab[0x0309] = Some("WM_PAINTCLIPBOARD");
    tab[0x030A] = Some("WM_VSCROLLCLIPBOARD");
    tab[0x030B] = Some("WM_SIZECLIPBOARD");
    tab[0x030C] = Some("WM_ASKCBFORMATNAME");
    tab[0x030D] = Some("WM_CHANGECBCHAIN");
    tab[0x030E] = Some("WM_HSCROLLCLIPBOARD");
    tab[0x030F] = Some("WM_QUERYNEWPALETTE");
    tab[0x0310] = Some("WM_PALETTEISCHANGING");
    tab[0x0311] = Some("WM_PALETTECHANGED");
    tab[0x0312] = Some("WM_HOTKEY");

    tab[0x0317] = Some("WM_PRINT");
    tab[0x0318] = Some("WM_PRINTCLIENT");
    tab[0x0319] = Some("WM_APPCOMMAND");
    tab[0x031A] = Some("WM_THEMECHANGED");
    tab[0x031D] = Some("WM_CLIPBOARDUPDATE");
    tab[0x0358] = Some("WM_HANDHELDFIRST");
    tab[0x035F] = Some("WM_HANDHELDLAST");

    tab[0x0360] = Some("WM_AFXFIRST");
    tab[0x037F] = Some("WM_AFXLAST");

    tab[0x0380] = Some("WM_PENWINFIRST");
    tab[0x038F] = Some("WM_PENWINLAST");
    tab[0x8000] = Some("WM_APP");
    tab
};
