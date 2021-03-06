use super::prelude::*;

#[no_mangle]
pub unsafe extern "C" fn luaopen_os(mut L: *mut lua_State) -> lua_int {
    luaL_checkversion_(
        L,
        503i32 as lua_Number,
        (::std::mem::size_of::<lua_Integer>() as lua_ulong)
            .wrapping_mul(16i32 as lua_ulong)
            .wrapping_add(::std::mem::size_of::<lua_Number>() as lua_ulong),
    );
    lua_createtable(
        L,
        0i32,
        (::std::mem::size_of::<[luaL_Reg; 12]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
    );
    luaL_setfuncs(L, syslib.as_ptr(), 0i32);
    return 1i32;
}
static mut syslib: [luaL_Reg; 12] = [
    lua_reg!(b"clock\x00", os_clock),
    lua_reg!(b"date\x00", os_date),
    lua_reg!(b"difftime\x00", os_difftime),
    lua_reg!(b"execute\x00", os_execute),
    lua_reg!(b"exit\x00", os_exit),
    lua_reg!(b"getenv\x00", os_getenv),
    lua_reg!(b"remove\x00", os_remove),
    lua_reg!(b"rename\x00", os_rename),
    lua_reg!(b"setlocale\x00", os_setlocale),
    lua_reg!(b"time\x00", os_time),
    lua_reg!(b"tmpname\x00", os_tmpname),
    lua_reg_none!(0),
];
unsafe extern "C" fn os_tmpname(mut L: *mut lua_State) -> lua_int {
    let mut buff: [lua_char; 32] = [0; 32];
    let mut err: lua_int = 0;
    strcpy(buff.as_mut_ptr(), s!(b"/tmp/lua_XXXXXX\x00"));
    err = mkstemp(buff.as_mut_ptr());
    if err != -1i32 {
        close(err);
    }
    err = (err == -1i32) as lua_int;
    if 0 != err {
        return luaL_error!(L, s!(b"unable to generate a unique filename\x00"));
    } else {
        lua_pushstring(L, buff.as_mut_ptr());
        return 1i32;
    };
}
unsafe extern "C" fn os_time(mut L: *mut lua_State) -> lua_int {
    let mut ts: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const lua_char,
    };
    let mut t: time_t = 0;
    /* called without args? */
    if lua_type(L, 1i32) <= 0i32 {
        /* get current time */
        t = time(0 as *mut time_t)
    } else {
        ts = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            __tm_gmtoff: 0,
            __tm_zone: 0 as *const lua_char,
        };
        luaL_checktype(L, 1i32, 5i32);
        /* make sure table is at the top */
        lua_settop(L, 1i32);
        ts.tm_sec = getfield(L, s!(b"sec\x00"), 0i32, 0i32);
        ts.tm_min = getfield(L, s!(b"min\x00"), 0i32, 0i32);
        ts.tm_hour = getfield(L, s!(b"hour\x00"), 12i32, 0i32);
        ts.tm_mday = getfield(L, s!(b"day\x00"), -1i32, 0i32);
        ts.tm_mon = getfield(L, s!(b"month\x00"), -1i32, 1i32);
        ts.tm_year = getfield(L, s!(b"year\x00"), -1i32, 1900i32);
        ts.tm_isdst = getboolfield(L, s!(b"isdst\x00"));
        t = mktime(&mut ts);
        /* update fields with normalized values */
        setallfields(L, &mut ts);
    }
    if t != t as lua_Integer as time_t || t == -1i32 as time_t {
        return luaL_error!(
            L,
            s!(b"time result cannot be represented in this installation\x00"),
        );
    } else {
        lua_pushinteger(L, t as lua_Integer);
        return 1i32;
    };
}
/*
** Set all fields from structure 'tm' in the table on top of the stack
*/
unsafe extern "C" fn setallfields(mut L: *mut lua_State, mut stm: *mut tm) -> () {
    setfield(L, s!(b"sec\x00"), (*stm).tm_sec);
    setfield(L, s!(b"min\x00"), (*stm).tm_min);
    setfield(L, s!(b"hour\x00"), (*stm).tm_hour);
    setfield(L, s!(b"day\x00"), (*stm).tm_mday);
    setfield(L, s!(b"month\x00"), (*stm).tm_mon + 1i32);
    setfield(L, s!(b"year\x00"), (*stm).tm_year + 1900i32);
    setfield(L, s!(b"wday\x00"), (*stm).tm_wday + 1i32);
    setfield(L, s!(b"yday\x00"), (*stm).tm_yday + 1i32);
    setboolfield(L, s!(b"isdst\x00"), (*stm).tm_isdst);
}
unsafe extern "C" fn setboolfield(
    mut L: *mut lua_State,
    mut key: *const lua_char,
    mut value: lua_int,
) -> () {
    /* undefined? */
    if value < 0i32 {
        /* does not set field */
        return;
    } else {
        lua_pushboolean(L, value);
        lua_setfield(L, -2i32, key);
        return;
    };
}
/*
** {======================================================
** Time/Date operations
** { year=%Y, month=%m, day=%d, hour=%H, min=%M, sec=%S,
**   wday=%w+1, yday=%j, isdst=? }
** =======================================================
*/
unsafe extern "C" fn setfield(
    mut L: *mut lua_State,
    mut key: *const lua_char,
    mut value: lua_int,
) -> () {
    lua_pushinteger(L, value as lua_Integer);
    lua_setfield(L, -2i32, key);
}
unsafe extern "C" fn getboolfield(mut L: *mut lua_State, mut key: *const lua_char) -> lua_int {
    let mut res: lua_int = 0;
    res = if lua_getfield(L, -1i32, key) == 0i32 {
        -1i32
    } else {
        lua_toboolean(L, -1i32)
    };
    lua_settop(L, -1i32 - 1i32);
    return res;
}
/* maximum value for date fields (to avoid arithmetic overflows with 'int') */
unsafe extern "C" fn getfield(
    mut L: *mut lua_State,
    mut key: *const lua_char,
    mut d: lua_int,
    mut delta: lua_int,
) -> lua_int {
    let mut isnum: lua_int = 0;
    /* get field and its type */
    let mut t: lua_int = lua_getfield(L, -1i32, key);
    let mut res: lua_Integer = lua_tointegerx(L, -1i32, &mut isnum);
    if 0 == isnum {
        /* field is not an integer? */
        /* some other value? */
        if t != 0i32 {
            return luaL_error!(L, s!(b"field \'%s\' is not an integer\x00"), key);
        } else if d < 0i32 {
            return luaL_error!(L, s!(b"field \'%s\' missing in date table\x00"), key);
        } else {
            res = d as lua_Integer
        }
    } else if !(-(2147483647i32 / 2i32) as lua_longlong <= res
        && res <= (2147483647i32 / 2i32) as lua_longlong)
    {
        return luaL_error!(L, s!(b"field \'%s\' is out-of-bound\x00"), key);
    } else {
        res -= delta as lua_longlong
    }
    lua_settop(L, -1i32 - 1i32);
    return res as lua_int;
}
/* }====================================================== */
unsafe extern "C" fn os_setlocale(mut L: *mut lua_State) -> lua_int {
    static mut cat: [lua_int; 6] = [6i32, 3i32, 0i32, 4i32, 1i32, 2i32];
    static mut catnames: [*const lua_char; 7] = [
        s!(b"all\x00"),
        s!(b"collate\x00"),
        s!(b"ctype\x00"),
        s!(b"monetary\x00"),
        s!(b"numeric\x00"),
        s!(b"time\x00"),
        0 as *const lua_char,
    ];
    let mut l: *const lua_char = luaL_optlstring(L, 1i32, 0 as *const lua_char, 0 as *mut size_t);
    let mut op: lua_int = luaL_checkoption(L, 2i32, s!(b"all\x00"), catnames.as_ptr());
    lua_pushstring(L, setlocale(cat[op as usize], l));
    return 1i32;
}
unsafe extern "C" fn os_rename(mut L: *mut lua_State) -> lua_int {
    let mut fromname: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut toname: *const lua_char = luaL_checklstring(L, 2i32, 0 as *mut size_t);
    return luaL_fileresult(
        L,
        (rename(fromname, toname) == 0i32) as lua_int,
        0 as *const lua_char,
    );
}
unsafe extern "C" fn os_remove(mut L: *mut lua_State) -> lua_int {
    let mut filename: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    return luaL_fileresult(L, (remove(filename) == 0i32) as lua_int, filename);
}
unsafe extern "C" fn os_getenv(mut L: *mut lua_State) -> lua_int {
    /* if NULL push nil */
    lua_pushstring(L, getenv(luaL_checklstring(L, 1i32, 0 as *mut size_t)));
    return 1i32;
}
unsafe extern "C" fn os_exit(mut L: *mut lua_State) -> lua_int {
    let mut status: lua_int = 0;
    if lua_type(L, 1i32) == 1i32 {
        status = if 0 != lua_toboolean(L, 1i32) {
            0i32
        } else {
            1i32
        }
    } else {
        status = luaL_optinteger(L, 1i32, 0i32 as lua_Integer) as lua_int
    }
    if 0 != lua_toboolean(L, 2i32) {
        lua_close(L);
    }
    if !L.is_null() {
        /* 'if' to avoid warnings for unreachable 'return' */
        exit(status);
    }
    return 0i32;
}

unsafe extern "C" fn os_execute(mut L: *mut lua_State) -> lua_int {
    let mut cmd: *const lua_char = luaL_optlstring(L, 1i32, 0 as *const lua_char, 0 as *mut size_t);
    let mut stat: lua_int = system(cmd);
    if !cmd.is_null() {
        return luaL_execresult(L, stat);
    } else {
        /* true if there is a shell */
        lua_pushboolean(L, stat);
        return 1i32;
    };
}
unsafe extern "C" fn os_difftime(mut L: *mut lua_State) -> lua_int {
    let mut t1: time_t = l_checktime(L, 1i32);
    let mut t2: time_t = l_checktime(L, 2i32);
    lua_pushnumber(L, difftime(t1, t2));
    return 1i32;
}

unsafe extern "C" fn l_checktime(mut L: *mut lua_State, mut arg: lua_int) -> time_t {
    let mut t: lua_Integer = luaL_checkinteger(L, arg);
    (t as time_t as lua_longlong == t || 0 != luaL_argerror(L, arg, s!(b"time out-of-bounds\x00")))
        as lua_int;
    return t as time_t;
}
/* maximum size for an individual 'strftime' item */
unsafe extern "C" fn os_date(mut L: *mut lua_State) -> lua_int {
    let mut slen: size_t = 0;
    let mut s: *const lua_char = luaL_optlstring(L, 1i32, s!(b"%c\x00"), &mut slen);
    let mut t: time_t = if lua_type(L, 2i32) <= 0i32 {
        time(0 as *mut time_t)
    } else {
        l_checktime(L, 2i32)
    };
    /* 's' end */
    let mut se: *const lua_char = s.offset(slen as isize);
    let mut tmr: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        __tm_gmtoff: 0,
        __tm_zone: 0 as *const lua_char,
    };
    let mut stm: *mut tm = 0 as *mut tm;
    if *s as lua_int == '!' as i32 {
        /* UTC? */
        stm = gmtime_r(&mut t, &mut tmr);
        /* skip '!' */
        s = s.offset(1isize)
    } else {
        stm = localtime_r(&mut t, &mut tmr)
    }
    /* invalid date? */
    if stm.is_null() {
        return luaL_error!(
            L,
            s!(b"time result cannot be represented in this installation\x00"),
        );
    } else {
        if strcmp(s, s!(b"*t\x00")) == 0i32 {
            /* 9 = number of fields */
            lua_createtable(L, 0i32, 9i32);
            setallfields(L, stm);
        } else {
            /* buffer for individual conversion specifiers */
            let mut cc: [lua_char; 4] = [0; 4];
            let mut b: luaL_Buffer = luaL_Buffer {
                b: 0 as *mut lua_char,
                size: 0,
                n: 0,
                L: 0 as *mut lua_State,
                initb: [0; 8192],
            };
            cc[0usize] = '%' as i32 as lua_char;
            luaL_buffinit(L, &mut b);
            while s < se {
                /* not a conversion specifier? */
                if *s as lua_int != '%' as i32 {
                    (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null())
                        as lua_int;
                    let fresh1 = b.n;
                    b.n = b.n.wrapping_add(1);
                    let fresh0 = s;
                    s = s.offset(1);
                    *b.b.offset(fresh1 as isize) = *fresh0
                } else {
                    let mut reslen: size_t = 0;
                    let mut buff: *mut lua_char = luaL_prepbuffsize(&mut b, 250i32 as size_t);
                    /* skip '%' */
                    s = s.offset(1isize);
                    /* copy specifier to 'cc' */
                    s = checkoption(
                        L,
                        s,
                        se.wrapping_offset_from(s) as lua_long,
                        cc.as_mut_ptr().offset(1isize),
                    );
                    reslen = strftime(buff, 250i32 as size_t, cc.as_mut_ptr(), stm);
                    b.n = (b.n as lua_ulong).wrapping_add(reslen) as size_t as size_t
                }
            }
            luaL_pushresult(&mut b);
        }
        return 1i32;
    };
}
unsafe extern "C" fn checkoption(
    mut L: *mut lua_State,
    mut conv: *const lua_char,
    mut convlen: ptrdiff_t,
    mut buff: *mut lua_char,
) -> *const lua_char {
    let mut option: *const lua_char =
        b"aAbBcCdDeFgGhHIjmMnprRStTuUVwWxXyYzZ%||EcECExEXEyEYOdOeOHOIOmOMOSOuOUOVOwOWOy\x00"
            as *const u8 as *const lua_char;
    /* length of options being checked */
    let mut oplen: lua_int = 1i32;
    while *option as lua_int != '\u{0}' as i32 && oplen as lua_long <= convlen {
        /* next block? */
        if *option as lua_int == '|' as i32 {
            /* will check options with next length (+1) */
            oplen += 1
        } else if memcmp(
            conv as *const lua_void,
            option as *const lua_void,
            oplen as lua_ulong,
        ) == 0i32
        {
            /* match? */
            /* copy valid option to buffer */
            memcpy(
                buff as *mut lua_void,
                conv as *const lua_void,
                oplen as lua_ulong,
            );
            *buff.offset(oplen as isize) = '\u{0}' as i32 as lua_char;
            /* return next item */
            return conv.offset(oplen as isize);
        }
        option = option.offset(oplen as isize)
    }
    luaL_argerror(
        L,
        1i32,
        lua_pushfstring!(L, s!(b"invalid conversion specifier \'%%%s\'\x00"), conv,),
    );
    /* to avoid warnings */
    return conv;
}
unsafe extern "C" fn os_clock(mut L: *mut lua_State) -> lua_int {
    lua_pushnumber(
        L,
        clock() as lua_Number / 1000000i32 as __clock_t as lua_Number,
    );
    return 1i32;
}
