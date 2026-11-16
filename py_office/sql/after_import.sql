PRAGMA FOREIGN_KEYS =OFF;
PRAGMA JOURNAL_MODE =OFF;
PRAGMA SYNCHRONOUS =OFF;

UPDATE raw SET unit=${number} WHERE unit=0;

INSERT INTO person(id,unit,total_payed)
SELECT id,unit,total_payed FROM raw;

INSERT INTO prizes(person_id, kind, cnt, reward) SELECT id,1,first_cnt,first_prize FROM raw WHERE first_cnt<>0;
INSERT INTO prizes(person_id, kind, cnt, reward) SELECT id,2,first_adjacent_cnt,first_adjacent_prize FROM raw WHERE first_adjacent_cnt<>0;
INSERT INTO prizes(person_id, kind, cnt, reward) SELECT id,3,first_different_group_cnt,first_different_group_prize FROM raw WHERE first_different_group_cnt<>0;
INSERT INTO prizes(person_id, kind, cnt, reward) SELECT id,4,second_cnt,second_prize FROM raw WHERE second_cnt<>0;
INSERT INTO prizes(person_id, kind, cnt, reward) SELECT id,5,third_cnt,third_prize FROM raw WHERE third_cnt<>0;
INSERT INTO prizes(person_id, kind, cnt, reward) SELECT id,6,fourth_cnt,fourth_prize FROM raw WHERE fourth_cnt<>0;
INSERT INTO prizes(person_id, kind, cnt, reward) SELECT id,7,fifth_cnt,fifth_prize FROM raw WHERE fifth_cnt<>0;
INSERT INTO prizes(person_id, kind, cnt, reward) SELECT id,8,sixth_cnt,sixth_prize FROM raw WHERE sixth_cnt<>0;
INSERT INTO prizes(person_id, kind, cnt, reward) SELECT id,9,seventh_cnt,seventh_prize FROM raw WHERE seventh_cnt<>0;

PRAGMA FOREIGN_KEY_CHECK ;
VACUUM ;