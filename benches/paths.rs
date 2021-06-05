use criterion::*;

use kars::*;

fn gen_tree(width: usize, height: usize) -> (Tree<'static>, usize) {
    let names: &'static mut Vec<String> = Box::leak(Box::new(vec![]));
    for i in 0..height {
        for j in 0..width {
            let name = format!("Node_{}_{}", i, j);
            names.push(name);
        }
    }

    let mut tree = Tree::new();
    let root = tree.add("Root", None);
    let mut parent = root;
    for i in 0..height {
        let mut children = Vec::new();
        for j in 0..width {
            let name = &names[width * i + j];
            children.push(tree.add(name, Some(parent)));
        }
        parent = children[width / 2];
    }
    (tree, parent)
}

fn paths_bench(c: &mut Criterion) {
    let (tree, leaf) = gen_tree(50, 50);
    c.bench_function("path", |b| b.iter(|| tree.path(leaf)));
    c.bench_function("r_path", |b| b.iter(|| tree.r_path(leaf)));
}

criterion_group!(benches, paths_bench);
criterion_main!(benches);
