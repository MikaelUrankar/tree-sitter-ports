Lapce expects libraries to be tree-sitter-rust and not libtree-sitter-rust
At least this is what it's in https://github.com/lapce/tree-sitter-grammars/releases/tag/v0.0.1


--- build-tree-sitter/src/main.rs.orig	2024-06-21 14:01:57
+++ build-tree-sitter/src/main.rs
@@ -252,7 +252,7 @@ fn build_tree_sitter_library(
         compiler.file(library_path.clone().join("scanner.o"));
     }
 
-    let mut out_lib = output.join(format!("lib{name}"));
+    let mut out_lib = output.join(format!("{name}"));
     out_lib.set_extension(std::env::consts::DLL_EXTENSION);
     compile(&mut compiler, out_lib.display().to_string())?;
 
