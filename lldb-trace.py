#!/usr/bin/env python3

"""
export PYTHONPATH=`lldb -P`
/Applications/Xcode.app/Contents/Developer/Library/Frameworks/Python3.framework/Versions/Current/bin/python3 -u trace.py | tee log
"""

import lldb
import os


RETROWIN32 = './target/x86_64-apple-darwin/debug/retrowin32'
TARGET_TRIPLE = 'x86_64-apple-macosx13.0.0'
EXE = 'exe/demo/effect.exe'

trace_points = []
with open('tp') as f:
    for line in f:
        if line.startswith('@'):
            trace_points.append(int(line.strip()[1:], 16))

debugger = lldb.SBDebugger.Create()
debugger.SetAsync(False)
target = debugger.CreateTargetWithFileAndTargetTriple(RETROWIN32, TARGET_TRIPLE)

bp = target.BreakpointCreateByName("jump_to_entry_point", target.GetExecutable().GetFilename())
err = lldb.SBError()
# Note target.LaunchSimple swallows stdout etc :(
process = target.Launch(
    lldb.SBListener(), ['--win32-trace', '*', EXE], None,
    '/dev/stdin', '/dev/stdout', '/dev/stderr', os.getcwd(),
    0, False, err
)

while True:
    bp.ClearAllBreakpointSites()
    thread = process.GetThreadAtIndex(0)
    frame = thread.GetFrameAtIndex(0)

    vals = ' '.join(
        '%s:%x' % ('e' + reg, frame.reg['r' + reg].unsigned & 0xFFFF_FFFF)
        for reg in ['ax', 'bx', 'cx', 'dx', 'si', 'di', 'sp']
    )
    print('@%x' % frame.reg['rip'].unsigned)
    print('  ' + vals)
    # Note: FPU state in lldb is 80-bit floats, and for the life of me I could not
    # get it to print those as anything other than arrays of bytes argh.
    #print('  fpu: ' + ' '.join(('%f' % frame.reg['stmm%d' % d].data.double[0]) for d in range(0,8)))

    next = trace_points.pop(0)
    bp = target.BreakpointCreateByAddress(next)
    process.Continue()

