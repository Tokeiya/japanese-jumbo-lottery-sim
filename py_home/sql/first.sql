SET SYNCHRONOUS_COMMIT = off;
SET session_replication_role = 'replica';
SET work_mem = '1GB';
SET maintenance_work_mem = '2GB';
SET temp_buffers = '2GB';
SET max_parallel_workers_per_gather = 4;
SET parallel_setup_cost = 0;
SET parallel_tuple_cost = 0;