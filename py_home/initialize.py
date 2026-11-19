from pathlib import Path

from psycopg import Connection, connect

from reader import gen_iterate


def insert_raw(conn: Connection, path: Path, unit: int):
    with conn.cursor() as cur:
        with cur.copy("""
                      COPY players_raw (unit, local_id, strategy, kind, cnt, reward)
                          FROM STDIN;
                      """) as cpy:
            for idx, elem in enumerate(gen_iterate(path)):
                tmp = (unit, *elem)

                if idx & 1023 == 0:
                    print(f"inserted {idx} rows")

                cpy.write_row(tmp)


def main():
    with connect(
            """postgresql://dell_eth:5432/lottery?options=-c%20synchronous_commit%3Doff%20-c%20session_replication_role%3Dreplica""") as conn:
        #        prepare_table(conn)
        print("main")
        insert_raw(conn, Path("../15output_players.tsv"), 6)

        # print("black")
        # insert_raw(conn, Path("./data/black_players_output.tsv"), 1)
        #
        # print("blue")
        # insert_raw(conn, Path("./data/blue_players_output.tsv"), 2)
        #
        # print("clear")
        # insert_raw(conn, Path("./data/clear_players_output.tsv"), 3)
        #
        # print("plastic")
        # insert_raw(conn, Path("./data/plastic_players_output.tsv"), 4)
        #
        # print("dell")
        # insert_raw(conn, Path("./data/blue_players_output.tsv"), 5)


if __name__ == "__main__":
    main()
