INSERT INTO player(id,strategy,unit,total_payed)
SELECT id,raw.strategy,unit,total_payed FROM raw;


INSERT INTO prize(player_id,kind,cnt,reward) SELECT id,'first',first_cnt,first_prize FROM raw WHERE first_cnt<>0;
INSERT INTO prize(player_id,kind,cnt,reward) SELECT id,'first_adjacent',first_adjacent_cnt,first_adjacent_prize FROM raw WHERE first_adjacent_cnt<>0;
INSERT INTO prize(player_id,kind,cnt,reward) SELECT id,'first_different_group',first_different_group_cnt,first_different_group_prize FROM raw WHERE first_different_group_cnt<>0;
INSERT INTO prize(player_id,kind,cnt,reward) SELECT id,'second',second_cnt,second_prize FROM raw WHERE second_cnt<>0;
INSERT INTO prize(player_id,kind,cnt,reward) SELECT id,'third',third_cnt,third_prize FROM raw WHERE third_cnt<>0;
INSERT INTO prize(player_id,kind,cnt,reward) SELECT id,'fourth',fourth_cnt,fourth_prize FROM raw WHERE fourth_cnt<>0;
INSERT INTO prize(player_id,kind,cnt,reward) SELECT id,'fifth',fifth_cnt,fifth_prize FROM raw WHERE fifth_cnt<>0;
INSERT INTO prize(player_id,kind,cnt,reward) SELECT id,'sixth',sixth_cnt,sixth_prize FROM raw WHERE sixth_cnt<>0;
INSERT INTO prize(player_id,kind,cnt,reward) SELECT id,'seventh',seventh_cnt,seventh_prize FROM raw WHERE seventh_cnt<>0;

CREATE INDEX idx_prize_player_id ON prize(player_id);
CREATE INDEX idx_prize_cnt ON prize(cnt);
CREATE INDEX idx_prize_kind ON prize(kind);
CREATE INDEX idx_prize_reward ON prize(reward);

CREATE INDEX idx_player_unit ON player(unit);