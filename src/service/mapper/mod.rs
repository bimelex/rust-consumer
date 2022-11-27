
pub mod agent_query {
    pub fn agent_insert_query() -> &str {
        "INSERT INTO AGENT_INFO_TEST \
                        (\
                        AGENT_ID \
                        , AGENT_NAME\
                        , CONNECT_YN\
                        , GROUP_ID\
                        )\
                        VALUES \
                        (\
                        $1\
                        , $2\
                        , $3\
                        , $4\
                        )"
    }
}


// pub fn realtime_insert_query() -> &str {
//
//
// }