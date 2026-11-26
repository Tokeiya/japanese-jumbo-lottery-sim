from pathlib import Path

from psycopg import Connection, connect

from reader import gen_iterate


def insert_raw(conn: Connection, path: Path, unit: int):
    with conn.cursor() as cur:
        with cur.copy("""
                      COPY raw_players (unit_id, local_id, strategy, prize, cnt, reward)
                          FROM STDIN;
                      """) as cpy:
            for idx, elem in enumerate(gen_iterate(path)):
                tmp = (unit, *elem)

                if idx & 1023 == 0:
                    print(f"inserted {idx} rows")

                cpy.write_row(tmp)


def main():
    data = {
        "black_1_players": 1,
        "blue_1_players": 2,
        "clear_1_players": 3,
        "dell1_1_players": 4,
        "ryzen_1_players": 5,
        "ryzen_2_players": 6,
        "ryzen_3_players": 7,
        "winarmdevel_1_players": 8,
        "winarmdevel_2_players": 9,
        "winarmdevel_3_players": 10
    }
    with connect(
            """postgresql://dell1:5432/lottery?options=-c%20synchronous_commit%3Doff%20-c%20session_replication_role%3Dreplica""") as conn:
        for file in Path("/mnt/wsl/data/lottery_source").glob("*players*.tsv"):
            unit_id = data[file.stem]
            print(f"{unit_id}:{file.name}")
            insert_raw(conn, Path(file.name), unit_id)


if __name__ == "__main__":
    main()
