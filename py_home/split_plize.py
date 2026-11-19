def split_prize():
    from urllib.parse import urlencode

    options = " ".join([
        "-c synchronous_commit=off",
        "-c session_replication_role=replica",
        "-c work_mem=1GB",
        "-c maintenance_work_mem=2GB",
        "-c temp_buffers=2GB",
        "-c max_parallel_workers_per_gather=4",
    ])

    uri = f"postgresql://blue:5432/lottery?{urlencode({'options': options})}"

    cat = ['first', 'first_adjacent', 'first_different_group', 'second', 'third', 'fourth', 'fifth', 'sixth', 'seventh']

    tmp = [(p, f'{p}_cnt', f'{p}_prize') for p in cat]

    for (cat, cnt, prize) in tmp:
        cmd = f"""INSERT INTO prizes(player_id,rank,cnt,reward) SELECT id,'{cat}',{cnt},{prize} FROM raw WHERE {cnt}<>0;"""
        print(cmd)

    # with psycopg.connect(uri) as conn:


if __name__ == '__main__':
    split_prize()
