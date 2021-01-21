config = {  
  "bin": [
    {
      "name": "usage",
      "title": "Usage",
      "path": "src/usage.rs"
    },
    {
      "name": "advanced",
      "title": "Advanced",
      "path": "src/advanced.rs"
    }
  ],
}

def dump_text(path, text):
  with open(path, 'w') as f:
    f.write(text)

def read_text(path):
  with open(path) as f:
    return f.read()

def decorate(text, prefix):
  return "\n".join([f"{prefix}{line}" for line in text.split("\n")])

docexamples = []
mdexamples = []
for cbin in config["bin"]:  
  code = read_text(cbin["path"])
  docexamples.append(f"//!\n//! ## {cbin['title']}\n//!\n//!```\n" + decorate(code, "//!") + "```\n//!\n") 
  mdexamples.append(f"# {cbin['title']}\n\n```rust\n" + code + "```\n") 
docexamples = "//!\n//!\n//! # Examples\n//!\n" + "".join(docexamples)
mdexamples = "\n".join(mdexamples)
lib = read_text("src/lib.rs").split("// lib")
lib = docexamples + "\n\n// lib" + lib[1]
dump_text("src/lib.rs", lib)
readme = read_text("s/ReadMe.md")
readme = readme + "\n\n" + mdexamples + """\n# Logging

```bash
export RUST_LOG=info
# or
export RUST_LOG=debug
```"""
dump_text("ReadMe.md", readme)

gitconfig = read_text("s/config")
dump_text(".git/config", gitconfig)
