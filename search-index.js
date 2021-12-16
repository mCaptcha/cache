var searchIndex = JSON.parse('{\
"cache":{"doc":"","t":[17,17,3,17,17,3,17,3,3,17,12,12,12,12,11,11,11,11,11,11,11,11,0,0,11,11,11,11,0,11,11,11,11,11,11,11,11,0,5,0,0,11,11,11,11,11,11,11,11,11,11,11,11,0,11,11,11,11,3,4,13,7,17,11,11,11,11,12,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,0,11,11,5,5,5,12,3,7,17,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,5,5,5,4,6,13,13,13,13,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,7,3,17,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,0,11,5,5,5,5,5,5,7,17,3,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,17,5,5,5,5,5,5,5,5,5,5,5,5,5],"n":["BUCKET_EXPIRY_OFFSET","HIT_PER_SECOND","ID","PKG_NAME","PKG_VERSION","PREFIX_BUCKET","PREFIX_BUCKET_TIMER","PREFIX_CAPTCHA","PREFIX_CHALLENGE","PREFIX_SAFETY","__private_field","__private_field","__private_field","__private_field","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","bucket","challenge","deref","deref","deref","deref","errors","from","from","from","from","into","into","into","into","mcaptcha","on_delete","redis","safety","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","utils","vzip","vzip","vzip","vzip","Bucket","Format","Json","MCAPTCHA_BUCKET_TYPE","REDIS_MCAPTCHA_BUCKET_TYPE_VERSION","borrow","borrow","borrow_mut","borrow_mut","bucket_instant","clone","clone_into","counter_create","decrement","decrement","decrement_runner","deserialize","eq","fmt","fmt","from","from","from_str","increment","increment_by","into","into","new","on_delete","parse_str","serialize","timer","to_owned","try_from","try_from","try_into","try_into","type_id","type_id","type_methods","vzip","vzip","free","rdb_load","rdb_save","0","Challenge","MCAPTCHA_CHALLENGE_TYPE","MCAPTCHA_CHALLENGE_VERSION","borrow","borrow_mut","create_challenge","delete_challenge","deserialize","from","get_challenge","into","new","serialize","try_from","try_into","type_id","type_methods","vzip","free","rdb_load","rdb_save","CacheError","CacheResult","CaptchaNotFound","ChallengeNotFound","DuplicateChallenge","Msg","RedisError","borrow","borrow_mut","fmt","fmt","from","from","from","from","from","from","from","into","new","to_string","try_from","try_into","type_id","vzip","0","0","MCAPTCHA_MCAPTCHA_TYPE","MCaptcha","REDIS_MCPATCHA_MCAPTCHA_TYPE_VERSION","add_captcha","add_captcha_runner","add_visitor","borrow","borrow_mut","captcha_exists","captcha_exists_runner","decrement_visitor_by","delete_captcha","delete_captcha_runner","deserialize","from","get_add_visitor_result","get_count","get_difficulty","get_duration","get_mcaptcha","get_mut_mcaptcha","get_visitors","into","m","new","rename","serialize","try_from","try_into","type_id","type_methods","vzip","free","rdb_load","rdb_save","RedisModule_OnLoad","RedisModule_OnUnload","__info_func","MCAPTCHA_SAFETY_TYPE","MCAPTCHA_SAFETY_VERSION","MCaptchaSafety","boost","borrow","borrow_mut","deserialize","from","into","new","on_delete","serialize","set_timer","try_from","try_into","type_id","type_methods","vzip","SAFETY_RDB_VAL","free","rdb_load","rdb_save","get_bucket_instant","get_bucket_name","get_bucket_name_from_timer_name","get_captcha_key","get_challenge_name","get_mcaptcha_from_safety","get_safety_name","get_timer_name_from_bucket_name","is_bucket_timer","is_mcaptcha_safety"],"q":["cache","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cache::bucket","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cache::bucket::type_methods","","","cache::challenge","","","","","","","","","","","","","","","","","","","cache::challenge::type_methods","","","cache::errors","","","","","","","","","","","","","","","","","","","","","","","","","cache::errors::CacheError","","cache::mcaptcha","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","cache::mcaptcha::type_methods","","","cache::redis","","","cache::safety","","","","","","","","","","","","","","","","","","cache::safety::type_methods","","","","cache::utils","","","","","","","","",""],"d":["If buckets perform clean up at x instant, then buckets …","Initial allocation amount of bucketbucket::Bucket","node unique identifier, useful when running in cluster mode","","","bucket key prefix","bucket timer key prefix","counter/captcha key prefix","","","","","","","","","","","","","","","Leaky bucket algorithm is implemantation for mcatpcha …","","","","","","","","","","","","","","","","","","Custom datastructure that controls mCaptcha lifetime …","","","","","","","","","","","","","","","","","","","encoding formats for persistence","","","Bucket type version, aka encoding version","","","","","instant(seconds from UNIX_EPOCH) at which time bucket …","","","Create new counter","executes when timer goes off. Decrements all registered …","a list of captcha keys that should be decremented during …","decrement runner that decrements all registered counts …","","","","","","","","increments count of key = captcha and registers for auto …","open bucket, set decrement by specified number","","","creates new bucket and sets off timer to go off at <code>duration</code>","Run when bucket timer expired at BUCKET_EXPIRY_OFFSET. …","","","timer ID","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Add captcha to redis","","increments the visitor count by one","","","check if captcha exists","","decrement MCaptcha’s current visitor_threshold by …","delete captcha","","","","","Get counter value","get current difficulty factor","get MCaptcha’s lifetime","get mcaptcha from redis key","get mcaptcha from redis key writable","get MCaptcha’s current visitor_threshold","","","","implements mCaptcha rename: clones configuration from old …","","","","","","","","","","","","","","","","executes when timer goes off. Refreshes expiry timer and …","","","","","","","When safety is deleted due to expiration, if mcaptcha …","","","","","","","","","","","","","duration in seconds","duration in seconds","","","","","duration in seconds","",""],"i":[0,0,0,0,0,0,0,0,0,0,1,2,3,4,1,2,3,4,1,2,3,4,0,0,1,2,3,4,0,1,2,3,4,1,2,3,4,0,0,0,0,1,2,3,4,1,2,3,4,1,2,3,4,0,1,2,3,4,0,0,5,0,0,5,6,5,6,6,6,6,6,6,6,6,6,5,5,6,5,6,5,6,6,5,6,6,6,5,6,6,6,5,6,5,6,5,6,0,5,6,0,0,0,7,0,0,0,7,7,7,7,7,7,7,7,7,7,7,7,7,0,7,0,0,0,0,0,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,8,9,10,0,0,0,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,0,11,0,0,0,0,0,0,0,0,0,12,12,12,12,12,12,12,12,12,12,12,12,12,0,12,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"f":[null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[],["usize",15]],[[],["string",3]],[[],["string",3]],[[],["string",3]],null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["notifyevent",3],["context",3],["str",15]]],null,null,[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],[[],["typeid",3]],null,[[]],[[]],[[]],[[]],null,null,null,null,null,[[]],[[]],[[]],[[]],null,[[],["bucket",3]],[[]],[[["vec",3,["redisstring"]],["context",3],["redisstring",3]],["redisresult",6]],[[["u64",15],["context",3]]],null,[[["context",3],["rediskeywritable",3]]],[[],["result",4]],[[["format",4]],["bool",15]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[]],[[]],[[["str",15]],[["deserialize",8],["result",4,["cacheerror"]],["cacheerror",4]]],[[["str",15],["context",3]],[["result",4,["string","cacheerror"]],["string",3],["cacheerror",4]]],[[["context",3],["u32",15]],[["result",4,["cacheerror"]],["cacheerror",4]]],[[]],[[]],[[["u64",15],["context",3]],[["cacheerror",4],["result",4,["cacheerror"]]]],[[["notifyevent",3],["context",3],["str",15]]],[[["str",15]],[["deserialize",8],["result",4,["cacheerror"]],["cacheerror",4]]],[[],["result",4]],null,[[]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[],["typeid",3]],null,[[]],[[]],[[]],[[["c_int",6]]],[[]],null,null,null,null,[[]],[[]],[[["vec",3,["redisstring"]],["context",3],["redisstring",3]],["redisresult",6]],[[["vec",3,["redisstring"]],["context",3],["redisstring",3]],["redisresult",6]],[[],["result",4]],[[]],[[["vec",3,["redisstring"]],["context",3],["redisstring",3]],["redisresult",6]],[[]],[[["u64",15],["u32",15]]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]],[[]],[[["c_int",6]]],[[]],null,null,null,null,null,null,null,[[]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["captchaerror",4]]],[[["parseinterror",3]]],[[["rediserror",4]]],[[["error",3]]],[[["string",3]]],[[]],[[["str",15]]],[[]],[[["string",3]]],[[],["string",3]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],[[]],null,null,null,null,null,[[["vec",3,["redisstring"]],["context",3],["redisstring",3]],["redisresult",6]],[[["str",15],["mcaptcha",3],["context",3]],["redisresult",6]],[[]],[[]],[[]],[[["vec",3,["redisstring"]],["context",3],["redisstring",3]],["redisresult",6]],[[["rediskey",3]],["bool",15]],[[["u32",15]]],[[["vec",3,["redisstring"]],["context",3],["redisstring",3]],["redisresult",6]],[[["str",15],["context",3]],["redisresult",6]],[[],["result",4]],[[]],[[],["addvisitorresult",3]],[[["vec",3,["redisstring"]],["context",3],["redisstring",3]],["redisresult",6]],[[],["u32",15]],[[],["u64",15]],[[["rediskey",3]],[["result",4,["option","cacheerror"]],["cacheerror",4],["option",4]]],[[["rediskeywritable",3]],[["result",4,["option","cacheerror"]],["cacheerror",4],["option",4]]],[[],["u32",15]],[[]],null,[[["createmcaptcha",3]],[["cacheerror",4],["result",4,["cacheerror"]]]],[[["vec",3,["redisstring"]],["context",3],["redisstring",3]],["redisresult",6]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]],[[]],[[["c_int",6]]],[[]],[[["c_int",6]],["c_int",6]],[[],["c_int",6]],[[["i32",15]]],null,null,null,[[["context",3]]],[[]],[[]],[[],["result",4]],[[]],[[]],[[["u64",15],["str",15],["context",3]],[["result",4,["cacheerror"]],["cacheerror",4]]],[[["notifyevent",3],["context",3],["str",15]]],[[],["result",4]],[[["context",3],["rediskeywritable",3]],[["result",4,["cacheerror"]],["cacheerror",4]]],[[],["result",4]],[[],["result",4]],[[],["typeid",3]],null,[[]],null,[[]],[[["c_int",6]]],[[]],[[["u64",15]],[["u64",15],["cacheerror",4],["result",4,["u64","cacheerror"]]]],[[["u64",15]],["string",3]],[[["str",15]],[["str",15],["option",4,["str"]]]],[[],["string",3]],[[["str",15]],["string",3]],[[["str",15]],[["str",15],["option",4,["str"]]]],[[["str",15]],["string",3]],[[["str",15]],["string",3]],[[["str",15]],["bool",15]],[[["str",15]],["bool",15]]],"p":[[3,"ID"],[3,"PREFIX_CAPTCHA"],[3,"PREFIX_BUCKET"],[3,"PREFIX_CHALLENGE"],[4,"Format"],[3,"Bucket"],[3,"Challenge"],[4,"CacheError"],[13,"Msg"],[13,"RedisError"],[3,"MCaptcha"],[3,"MCaptchaSafety"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};