//! Dumping a datastore to log messages and back.

use std::sync::atomic::{AtomicBool, Ordering};

use itertools::Itertools;
use re_arrow_store::{test_row, DataStore, DataStoreStats, TimeInt, TimeRange, Timeline};
use re_log_types::{
    component_types::InstanceKey,
    datagen::{
        build_frame_nr, build_log_time, build_some_colors, build_some_instances, build_some_point2d,
    },
    Component as _, DataTable, EntityPath, TableId,
};

// --- Dump ---

#[test]
fn data_store_dump() {
    init_logs();

    for mut config in re_arrow_store::test_util::all_configs() {
        // NOTE: insert IDs aren't serialized and can be different across runs.
        config.store_insert_ids = false;

        let mut store1 = DataStore::new(InstanceKey::name(), config.clone());
        let mut store2 = DataStore::new(InstanceKey::name(), config.clone());
        let mut store3 = DataStore::new(InstanceKey::name(), config.clone());

        data_store_dump_impl(&mut store1, &mut store2, &mut store3);
    }
}

fn data_store_dump_impl(store1: &mut DataStore, store2: &mut DataStore, store3: &mut DataStore) {
    // helper to insert a table both as a temporal and timeless payload
    let insert_table = |store: &mut DataStore, table: &DataTable| {
        // insert temporal
        store.insert_table(table).unwrap();

        // insert timeless
        let mut table_timeless = table.clone();
        table_timeless.col_timelines = Default::default();
        store.insert_table(&table_timeless).unwrap();
    };

    let ent_paths = ["this/that", "other", "yet/another/one"];
    let tables = ent_paths
        .iter()
        .map(|ent_path| create_insert_table(*ent_path))
        .collect_vec();

    // Fill the first store.
    for table in &tables {
        insert_table(store1, table);
    }
    if let err @ Err(_) = store1.sanity_check() {
        store1.sort_indices_if_needed();
        eprintln!("{store1}");
        err.unwrap();
    }

    // Dump the first store into the second one.
    for table in store1.to_data_tables(None) {
        store2.insert_table(&table).unwrap();
    }
    if let err @ Err(_) = store2.sanity_check() {
        store2.sort_indices_if_needed();
        eprintln!("{store2}");
        err.unwrap();
    }

    // Dump the second store into the third one.
    for table in store2.to_data_tables(None) {
        store3.insert_table(&table).unwrap();
    }
    if let err @ Err(_) = store3.sanity_check() {
        store3.sort_indices_if_needed();
        eprintln!("{store3}");
        err.unwrap();
    }

    let store1_df = store1.to_dataframe();
    let store2_df = store2.to_dataframe();
    let store3_df = store3.to_dataframe();
    assert!(
        store1_df == store2_df,
        "First & second stores differ:\n{store1_df}\n{store2_df}"
    );
    assert!(
        store1_df == store3_df,
        "First & third stores differ:\n{store1_df}\n{store3_df}"
    );

    let store1_stats = DataStoreStats::from_store(store1);
    let store2_stats = DataStoreStats::from_store(store2);
    let store3_stats = DataStoreStats::from_store(store3);
    assert!(
        store1_stats <= store2_stats,
        "First store should have <= amount of data of second store:\n\
            {store1_stats:#?}\n{store2_stats:#?}"
    );
    assert!(
        store2_stats <= store3_stats,
        "Second store should have <= amount of data of third store:\n\
            {store2_stats:#?}\n{store3_stats:#?}"
    );
}

// --- Time-based filtering ---

#[test]
fn data_store_dump_filtered() {
    init_logs();

    for mut config in re_arrow_store::test_util::all_configs() {
        // NOTE: insert IDs aren't serialized and can be different across runs.
        config.store_insert_ids = false;

        let mut store1 = DataStore::new(InstanceKey::name(), config.clone());
        let mut store2 = DataStore::new(InstanceKey::name(), config.clone());

        data_store_dump_filtered_impl(&mut store1, &mut store2);
    }
}

fn data_store_dump_filtered_impl(store1: &mut DataStore, store2: &mut DataStore) {
    let timeline_frame_nr = Timeline::new_sequence("frame_nr");
    let timeline_log_time = Timeline::new_temporal("log_time");
    let frame1: TimeInt = 1.into();
    let frame2: TimeInt = 2.into();
    let frame3: TimeInt = 3.into();
    let frame4: TimeInt = 4.into();

    let ent_paths = ["this/that", "other", "yet/another/one"];
    let tables = ent_paths
        .iter()
        .map(|ent_path| create_insert_table(*ent_path))
        .collect_vec();

    // Fill the first store.
    for table in &tables {
        store1.insert_table(table).unwrap();
    }
    if let err @ Err(_) = store1.sanity_check() {
        store1.sort_indices_if_needed();
        eprintln!("{store1}");
        err.unwrap();
    }

    // Dump frame1 from the first store into the second one.
    for table in store1.to_data_tables((timeline_frame_nr, TimeRange::new(frame1, frame1)).into()) {
        store2.insert_table(&table).unwrap();
    }
    // Dump frame2 from the first store into the second one.
    for table in store1.to_data_tables((timeline_frame_nr, TimeRange::new(frame2, frame2)).into()) {
        store2.insert_table(&table).unwrap();
    }
    // Dump frame3 from the first store into the second one.
    for table in store1.to_data_tables((timeline_frame_nr, TimeRange::new(frame3, frame3)).into()) {
        store2.insert_table(&table).unwrap();
    }
    // Dump the other frame3 from the first store into the second one.
    for table in store1.to_data_tables((timeline_log_time, TimeRange::new(frame3, frame3)).into()) {
        store2.insert_table(&table).unwrap();
    }
    // Dump frame4 from the first store into the second one.
    for table in store1.to_data_tables((timeline_frame_nr, TimeRange::new(frame4, frame4)).into()) {
        store2.insert_table(&table).unwrap();
    }
    if let err @ Err(_) = store2.sanity_check() {
        store2.sort_indices_if_needed();
        eprintln!("{store2}");
        err.unwrap();
    }

    let store1_df = store1.to_dataframe();
    let store2_df = store2.to_dataframe();
    assert!(
        store1_df == store2_df,
        "First & second stores differ:\n{store1_df}\n{store2_df}"
    );

    let store1_stats = DataStoreStats::from_store(store1);
    let store2_stats = DataStoreStats::from_store(store2);
    assert!(
        store1_stats <= store2_stats,
        "First store should have <= amount of data of second store:\n\
            {store1_stats:#?}\n{store2_stats:#?}"
    );
}

// ---

pub fn init_logs() {
    static INIT: AtomicBool = AtomicBool::new(false);

    if INIT
        .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        re_log::setup_native_logging();
    }
}

fn create_insert_table(ent_path: impl Into<EntityPath>) -> DataTable {
    let ent_path = ent_path.into();

    let frame1: TimeInt = 1.into();
    let frame2: TimeInt = 2.into();
    let frame3: TimeInt = 3.into();
    let frame4: TimeInt = 4.into();

    let (instances1, colors1) = (build_some_instances(3), build_some_colors(3));
    let row1 = test_row!(ent_path @ [
            build_frame_nr(frame1),
        ] => 3; [instances1.clone(), colors1]);

    let points2 = build_some_point2d(3);
    let row2 = test_row!(ent_path @ [
            build_frame_nr(frame2),
        ] => 3; [instances1, points2]);

    let points3 = build_some_point2d(10);
    let row3 = test_row!(ent_path @ [
            build_log_time(frame3.into()) /* ! */, build_frame_nr(frame3),
        ] => 10; [points3]);

    let colors4 = build_some_colors(5);
    let row4 = test_row!(ent_path @ [
            build_frame_nr(frame4),
        ] => 5; [colors4]);

    let mut table = DataTable::from_rows(TableId::random(), [row1, row2, row3, row4]);
    table.compute_all_size_bytes();

    table
}
