use std::sync::Arc;

use engula::*;

#[tokio::main]
async fn main() -> Result<()> {
    let dirname = "/tmp/engula_test/hello";
    let _ = std::fs::remove_dir_all(dirname);

    let options = Options::default();
    let journal_options = JournalOptions::default();
    let sstable_options = SstableOptions::default();
    let manifest_options = ManifestOptions::default();

    let journal = Arc::new(LocalJournal::new(dirname, journal_options)?);
    let fs = Arc::new(LocalFs::new(dirname)?);
    let storage = Arc::new(SstableStorage::new(fs, sstable_options));
    let runtime = Arc::new(LocalCompaction::new(storage.clone()));
    let manifest = Arc::new(LocalManifest::new(
        manifest_options,
        storage.clone(),
        runtime,
    ));
    let db = Arc::new(Database::new(options, journal, storage, manifest).await?);

    let num_tasks = 4u64;
    let num_entries = 1024u64;
    let mut tasks = Vec::new();
    for _ in 0..num_tasks {
        let db = db.clone();
        let task = tokio::task::spawn(async move {
            for i in 0..num_entries {
                let v = i.to_be_bytes().to_vec();
                db.put(v.clone(), v.clone()).await.unwrap();
                let got = db.get(&v).await.unwrap();
                assert_eq!(got, Some(v.clone()));
            }
        });
        tasks.push(task);
    }
    for task in tasks {
        task.await?;
    }
    for i in 0..num_entries {
        let v = i.to_be_bytes().to_vec();
        let got = db.get(&v).await.unwrap();
        assert_eq!(got, Some(v.clone()));
    }
    db.count().await.unwrap();
    Ok(())
}
