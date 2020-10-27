import glob
import os
import subprocess as sub

from pprint import pprint
from string import Template
from shlex import split
from typing import *

path = os.path

class Rule(NamedTuple):
    template: Template
    
    @classmethod
    def new(cls, tpl):
        return cls(template=Template(tpl))
    
    def call(self, *args, **kwargs):
        sub.call(split(self.template.substitute(**kwargs)))

    
cc_obj = Rule.new("${cc} ${cflags} -c ${src} -o ${out}")
cc_exe = Rule.new("${cc} ${ldflags} -o ${out} ${src}")

def join_fmt(fmt, iterable):
    return " ".join(fmt % elem for elem in iterable)

def main():
    build_dir = "build"
    
    srcs = glob.glob("*.c")
    objs = tuple(
        path.join(build_dir, path.splitext(path.basename(src))[0] + ".o")
        for src in srcs
    )
    
    os.makedirs(build_dir, exist_ok=True)
    
    defaults = {
        "cc": "gcc",
        "cflags": "-Wall -Wextra -Werror",
        "ldflags": "",
    }
    
    include_dirs = [
        "../include"
    ]
    
    link_dirs = [
        "../target/debug",
    ]
    
    libs = [
        # "chelp",
    ]
    
    env = {
        key: os.getenv(key.upper(), default=val)
        for key, val in defaults.items()
    }
    
    env["cflags"] += " %s" % join_fmt("-I%s", include_dirs)

    env["ldflags"] = "%s %s %s" % (
        join_fmt("-L%s", link_dirs),
        join_fmt("-l%s", libs),
        env["ldflags"],
    )
    
    pprint(env)
    # exit()
    
    for src, obj in zip(srcs, objs):
        cc_obj.call(**env, src=src, out=obj)
    
    objs += (path.join(link_dirs[0], "libchelp.so"),)
    
    cc_exe.call(**env,
        src=" ".join(objs),
        out=path.join(build_dir, "test")
    )
    
if __name__ == "__main__":
    main()    
