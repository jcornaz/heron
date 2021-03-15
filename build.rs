use skeptic;

fn main() {
    let mut files = skeptic::markdown_files_of_directory("guide/src");
    files.push("README.md".into());
    skeptic::generate_doc_tests(&files);
}
