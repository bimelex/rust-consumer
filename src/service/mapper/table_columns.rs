pub mod table_columns {
    pub fn get_agent_info () -> Vec<&str>{
        let mut columns = Vec::new();
        columns.push("agent_id");
        columns.push("agent_name");
        columns.push("connect_yn");
        columns.push("group_id");

        columns

    }

    pub fn get_real_time_perf () -> Vec<&str> {

        let mut columns = Vec::new();

        columns.push("agent_id");
        columns.push("agent_name");
        columns.push("ontune_time");
        columns.push("_user");
        columns.push("sys");
        columns.push("idle");
        columns.push("processor_count");
        columns.push("run_queue");
        columns.push("block_queue");
        columns.push("wait_queue");
        columns.push("p_queue");
        columns.push("p_crate_user");
        columns.push("p_crate_sys");
        columns.push("memory_size");
        columns.push("memory_used");
        columns.push("memory_pinned");
        columns.push("memory_sys");
        columns.push("memory_user");
        columns.push("memory_cache");
        columns.push("avm");
        columns.push("paging_space_in");
        columns.push("paging_space_out");
        columns.push("file_system_in");
        columns.push("file_system_out");
        columns.push("memory_scan");
        columns.push("memory_freed");
        columns.push("swap_size");
        columns.push("swap_used");
        columns.push("swap_active");
        columns.push("fork");
        columns.push("EXEC");
        columns.push("interrupt");
        columns.push("system_call");
        columns.push("context_switch");
        columns.push("semaphore");
        columns.push("msg");
        columns.push("disk_read_write");
        columns.push("disk_iops");
        columns.push("network_read_write");
        columns.push("network_iops");
        columns.push("top_command_id");
        columns.push("top_command_count");
        columns.push("top_user_id");
        columns.push("top_cpu");
        columns.push("top_disk_id");
        columns.push("top_vg_id");
        columns.push("top_busy");
        columns.push("max_pid");
        columns.push("thread_count");
        columns.push("pid_count");
        columns.push("linux_buffer");
        columns.push("linux_cached");
        columns.push("linux_srec");
        columns.push("mem_used_mb");
        columns.push("irq");
        columns.push("soft_irq");
        columns.push("swap_used_mb");
        columns.push("dusm");
        columns.push("create_date");

        columns
    }
}