use super::prelude::*;

#[no_mangle]
pub unsafe extern "C" fn luaopen_debug(mut L: *mut lua_State) -> lua_int {
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
        (::std::mem::size_of::<[luaL_Reg; 17]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<luaL_Reg>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong) as lua_int,
    );
    luaL_setfuncs(L, dblib.as_ptr(), 0i32);
    return 1i32;
}
static mut dblib: [luaL_Reg; 17] = [
    lua_reg!(b"debug\x00", db_debug),
    lua_reg!(b"getuservalue\x00", db_getuservalue),
    lua_reg!(b"gethook\x00", db_gethook),
    lua_reg!(b"getinfo\x00", db_getinfo),
    lua_reg!(b"getlocal\x00", db_getlocal),
    lua_reg!(b"getregistry\x00", db_getregistry),
    lua_reg!(b"getmetatable\x00", db_getmetatable),
    lua_reg!(b"getupvalue\x00", db_getupvalue),
    lua_reg!(b"upvaluejoin\x00", db_upvaluejoin),
    lua_reg!(b"upvalueid\x00", db_upvalueid),
    lua_reg!(b"setuservalue\x00", db_setuservalue),
    lua_reg!(b"sethook\x00", db_sethook),
    lua_reg!(b"setlocal\x00", db_setlocal),
    lua_reg!(b"setmetatable\x00", db_setmetatable),
    lua_reg!(b"setupvalue\x00", db_setupvalue),
    lua_reg!(b"traceback\x00", db_traceback),
    lua_reg_none!(0),
];
unsafe extern "C" fn db_traceback(mut L: *mut lua_State) -> lua_int {
    let mut level: lua_int = 0;
    let mut arg: lua_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut msg: *const lua_char = lua_tolstring(L, arg + 1i32, 0 as *mut size_t);
    /* non-string 'msg'? */
    if msg.is_null() && !(lua_type(L, arg + 1i32) <= 0i32) {
        /* return it untouched */
        lua_pushvalue(L, arg + 1i32);
    } else {
        level = luaL_optinteger(
            L,
            arg + 2i32,
            (if L == L1 { 1i32 } else { 0i32 }) as lua_Integer,
        ) as lua_int;
        luaL_traceback(L, L1, msg, level);
    }
    return 1i32;
}
/*
** Auxiliary function used by several library functions: check for
** an optional thread as function's first argument and set 'arg' with
** 1 if this argument is present (so that functions can skip it to
** access their other arguments)
*/
unsafe extern "C" fn getthread(mut L: *mut lua_State, mut arg: *mut lua_int) -> *mut lua_State {
    if lua_type(L, 1i32) == 8i32 {
        *arg = 1i32;
        return lua_tothread(L, 1i32);
    } else {
        *arg = 0i32;
        /* function will operate over current thread */
        return L;
    };
}
unsafe extern "C" fn db_setupvalue(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 3i32);
    return auxupvalue(L, 0i32);
}
/*
** get (if 'get' is true) or set an upvalue from a closure
*/
unsafe extern "C" fn auxupvalue(mut L: *mut lua_State, mut get: lua_int) -> lua_int {
    let mut name: *const lua_char = 0 as *const lua_char;
    /* upvalue index */
    let mut n: lua_int = luaL_checkinteger(L, 2i32) as lua_int;
    /* closure */
    luaL_checktype(L, 1i32, 6i32);
    name = if 0 != get {
        lua_getupvalue(L, 1i32, n)
    } else {
        lua_setupvalue(L, 1i32, n)
    };
    if name.is_null() {
        return 0i32;
    } else {
        lua_pushstring(L, name);
        /* no-op if get is false */
        lua_rotate(L, -(get + 1i32), 1i32);
        return get + 1i32;
    };
}
unsafe extern "C" fn db_setmetatable(mut L: *mut lua_State) -> lua_int {
    let mut t: lua_int = lua_type(L, 2i32);
    (t == 0i32 || t == 5i32 || 0 != luaL_argerror(L, 2i32, s!(b"nil or table expected\x00")))
        as lua_int;
    lua_settop(L, 2i32);
    lua_setmetatable(L, 1i32);
    /* return 1st argument */
    return 1i32;
}
unsafe extern "C" fn db_setlocal(mut L: *mut lua_State) -> lua_int {
    let mut arg: lua_int = 0;
    let mut name: *const lua_char = 0 as *const lua_char;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const lua_char,
        namewhat: 0 as *const lua_char,
        what: 0 as *const lua_char,
        source: 0 as *const lua_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut level: lua_int = luaL_checkinteger(L, arg + 1i32) as lua_int;
    let mut nvar: lua_int = luaL_checkinteger(L, arg + 2i32) as lua_int;
    /* out of range? */
    if 0 == lua_getstack(L1, level, &mut ar) {
        return luaL_argerror(L, arg + 1i32, s!(b"level out of range\x00"));
    } else {
        luaL_checkany(L, arg + 3i32);
        lua_settop(L, arg + 3i32);
        checkstack(L, L1, 1i32);
        lua_xmove(L, L1, 1i32);
        name = lua_setlocal(L1, &mut ar, nvar);
        if name.is_null() {
            /* pop value (if not popped by 'lua_setlocal') */
            lua_settop(L1, -1i32 - 1i32);
        }
        lua_pushstring(L, name);
        return 1i32;
    };
}
/*
** If L1 != L, L1 can be in any state, and therefore there are no
** guarantees about its stack space; any push in L1 must be
** checked.
*/
unsafe extern "C" fn checkstack(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut n: lua_int,
) -> () {
    if L != L1 && 0 == lua_checkstack(L1, n) {
        luaL_error!(L, s!(b"stack overflow\x00"));
    };
}
unsafe extern "C" fn db_sethook(mut L: *mut lua_State) -> lua_int {
    let mut smask: *const lua_char = 0 as *const lua_char;
    let mut arg: lua_int = 0;
    let mut mask: lua_int = 0;
    let mut count: lua_int = 0;
    let mut func: lua_Hook = None;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    if lua_type(L, arg + 1i32) <= 0i32 {
        /* no hook? */
        lua_settop(L, arg + 1i32);
        func = None;
        mask = 0i32;
        /* turn off hooks */
        count = 0i32
    } else {
        smask = luaL_checklstring(L, arg + 2i32, 0 as *mut size_t);
        luaL_checktype(L, arg + 1i32, 6i32);
        count = luaL_optinteger(L, arg + 3i32, 0i32 as lua_Integer) as lua_int;
        func = Some(hookf);
        mask = makemask(smask, count)
    }
    if lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &HOOKKEY as *const lua_int as *const lua_void,
    ) == 0i32
    {
        /* create a hook table */
        lua_createtable(L, 0i32, 2i32);
        lua_pushvalue(L, -1i32);
        /* set it in position */
        lua_rawsetp(
            L,
            -1000000i32 - 1000i32,
            &HOOKKEY as *const lua_int as *const lua_void,
        );
        lua_pushstring(L, s!(b"k\x00"));
        /* * hooktable.__mode = "k" */
        lua_setfield(L, -2i32, s!(b"__mode\x00"));
        lua_pushvalue(L, -1i32);
        /* setmetatable(hooktable) = hooktable */
        lua_setmetatable(L, -2i32);
    }
    checkstack(L, L1, 1i32);
    lua_pushthread(L1);
    /* key (thread) */
    lua_xmove(L1, L, 1i32);
    /* value (hook function) */
    lua_pushvalue(L, arg + 1i32);
    /* hooktable[L1] = new Lua hook */
    lua_rawset(L, -3i32);
    lua_sethook(L1, func, mask, count);
    return 0i32;
}
/*
** $Id: ldblib.c,v 1.151.1.1 2017/04/19 17:20:42 roberto Exp $
** Interface from Lua to its debug API
** See Copyright Notice in lua.h
*/
/*
** The hook table at registry[&HOOKKEY] maps threads to their current
** hook function. (We only need the unique address of 'HOOKKEY'.)
*/
static mut HOOKKEY: lua_int = 0i32;
/*
** Convert a string mask (for 'sethook') into a bit mask
*/
unsafe extern "C" fn makemask(mut smask: *const lua_char, mut count: lua_int) -> lua_int {
    let mut mask: lua_int = 0i32;
    if !strchr(smask, 'c' as i32).is_null() {
        mask |= 1i32 << 0i32
    }
    if !strchr(smask, 'r' as i32).is_null() {
        mask |= 1i32 << 1i32
    }
    if !strchr(smask, 'l' as i32).is_null() {
        mask |= 1i32 << 2i32
    }
    if count > 0i32 {
        mask |= 1i32 << 3i32
    }
    return mask;
}
/*
** Call hook function registered at hook table for the current
** thread (if there is one)
*/
unsafe extern "C" fn hookf(mut L: *mut lua_State, mut ar: *mut lua_Debug) -> () {
    static mut hooknames: [*const lua_char; 5] = [
        s!(b"call\x00"),
        s!(b"return\x00"),
        s!(b"line\x00"),
        s!(b"count\x00"),
        s!(b"tail call\x00"),
    ];
    lua_rawgetp(
        L,
        -1000000i32 - 1000i32,
        &HOOKKEY as *const lua_int as *const lua_void,
    );
    lua_pushthread(L);
    if lua_rawget(L, -2i32) == 6i32 {
        /* is there a hook function? */
        /* push event name */
        lua_pushstring(L, hooknames[(*ar).event as usize]);
        if (*ar).currentline >= 0i32 {
            /* push current line */
            lua_pushinteger(L, (*ar).currentline as lua_Integer);
        } else {
            lua_pushnil(L);
        }
        /* call hook function */
        lua_callk(L, 2i32, 0i32, 0i32 as lua_KContext, None);
    };
}
unsafe extern "C" fn db_setuservalue(mut L: *mut lua_State) -> lua_int {
    luaL_checktype(L, 1i32, 7i32);
    luaL_checkany(L, 2i32);
    lua_settop(L, 2i32);
    lua_setuservalue(L, 1i32);
    return 1i32;
}
unsafe extern "C" fn db_upvalueid(mut L: *mut lua_State) -> lua_int {
    let mut n: lua_int = checkupval(L, 1i32, 2i32);
    lua_pushlightuserdata(L, lua_upvalueid(L, 1i32, n));
    return 1i32;
}
/*
** Check whether a given upvalue from a given closure exists and
** returns its index
*/
unsafe extern "C" fn checkupval(
    mut L: *mut lua_State,
    mut argf: lua_int,
    mut argnup: lua_int,
) -> lua_int {
    /* upvalue index */
    let mut nup: lua_int = luaL_checkinteger(L, argnup) as lua_int;
    /* closure */
    luaL_checktype(L, argf, 6i32);
    (!lua_getupvalue(L, argf, nup).is_null()
        || 0 != luaL_argerror(L, argnup, s!(b"invalid upvalue index\x00"))) as lua_int;
    return nup;
}
unsafe extern "C" fn db_upvaluejoin(mut L: *mut lua_State) -> lua_int {
    let mut n1: lua_int = checkupval(L, 1i32, 2i32);
    let mut n2: lua_int = checkupval(L, 3i32, 4i32);
    (0 == lua_iscfunction(L, 1i32) || 0 != luaL_argerror(L, 1i32, s!(b"Lua function expected\x00")))
        as lua_int;
    (0 == lua_iscfunction(L, 3i32) || 0 != luaL_argerror(L, 3i32, s!(b"Lua function expected\x00")))
        as lua_int;
    lua_upvaluejoin(L, 1i32, n1, 3i32, n2);
    return 0i32;
}
unsafe extern "C" fn db_getupvalue(mut L: *mut lua_State) -> lua_int {
    return auxupvalue(L, 1i32);
}
unsafe extern "C" fn db_getmetatable(mut L: *mut lua_State) -> lua_int {
    luaL_checkany(L, 1i32);
    if 0 == lua_getmetatable(L, 1i32) {
        /* no metatable */
        lua_pushnil(L);
    }
    return 1i32;
}
unsafe extern "C" fn db_getregistry(mut L: *mut lua_State) -> lua_int {
    lua_pushvalue(L, -1000000i32 - 1000i32);
    return 1i32;
}
unsafe extern "C" fn db_getlocal(mut L: *mut lua_State) -> lua_int {
    let mut arg: lua_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const lua_char,
        namewhat: 0 as *const lua_char,
        what: 0 as *const lua_char,
        source: 0 as *const lua_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut name: *const lua_char = 0 as *const lua_char;
    /* local-variable index */
    let mut nvar: lua_int = luaL_checkinteger(L, arg + 2i32) as lua_int;
    if lua_type(L, arg + 1i32) == 6i32 {
        /* function argument? */
        /* push function */
        lua_pushvalue(L, arg + 1i32);
        /* push local name */
        lua_pushstring(L, lua_getlocal(L, 0 as *const lua_Debug, nvar));
        /* return only name (there is no value) */
        return 1i32;
    } else {
        /* stack-level argument */
        let mut level: lua_int = luaL_checkinteger(L, arg + 1i32) as lua_int;
        /* out of range? */
        if 0 == lua_getstack(L1, level, &mut ar) {
            return luaL_argerror(L, arg + 1i32, s!(b"level out of range\x00"));
        } else {
            checkstack(L, L1, 1i32);
            name = lua_getlocal(L1, &mut ar, nvar);
            if !name.is_null() {
                /* move local value */
                lua_xmove(L1, L, 1i32);
                /* push name */
                lua_pushstring(L, name);
                /* re-order */
                lua_rotate(L, -2i32, 1i32);
                return 2i32;
            } else {
                /* no name (nor value) */
                lua_pushnil(L);
                return 1i32;
            }
        }
    };
}
/*
** Calls 'lua_getinfo' and collects all results in a new table.
** L1 needs stack space for an optional input (function) plus
** two optional outputs (function and line table) from function
** 'lua_getinfo'.
*/
unsafe extern "C" fn db_getinfo(mut L: *mut lua_State) -> lua_int {
    let mut ar: lua_Debug = lua_Debug {
        event: 0,
        name: 0 as *const lua_char,
        namewhat: 0 as *const lua_char,
        what: 0 as *const lua_char,
        source: 0 as *const lua_char,
        currentline: 0,
        linedefined: 0,
        lastlinedefined: 0,
        nups: 0,
        nparams: 0,
        isvararg: 0,
        istailcall: 0,
        short_src: [0; 60],
        i_ci: 0 as *mut CallInfo,
    };
    let mut arg: lua_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut options: *const lua_char =
        luaL_optlstring(L, arg + 2i32, s!(b"flnStu\x00"), 0 as *mut size_t);
    checkstack(L, L1, 3i32);
    if lua_type(L, arg + 1i32) == 6i32 {
        /* info about a function? */
        /* add '>' to 'options' */
        options = lua_pushfstring!(L, s!(b">%s\x00"), options);
        /* move function to 'L1' stack */
        lua_pushvalue(L, arg + 1i32);
        lua_xmove(L, L1, 1i32);
    } else if 0 == lua_getstack(L1, luaL_checkinteger(L, arg + 1i32) as lua_int, &mut ar) {
        /* level out of range */
        lua_pushnil(L);
        return 1i32;
    }
    if 0 == lua_getinfo(L1, options, &mut ar) {
        return luaL_argerror(L, arg + 2i32, s!(b"invalid option\x00"));
    } else {
        /* table to collect results */
        lua_createtable(L, 0i32, 0i32);
        if !strchr(options, 'S' as i32).is_null() {
            settabss(L, s!(b"source\x00"), ar.source);
            settabss(L, s!(b"short_src\x00"), ar.short_src.as_mut_ptr());
            settabsi(L, s!(b"linedefined\x00"), ar.linedefined);
            settabsi(L, s!(b"lastlinedefined\x00"), ar.lastlinedefined);
            settabss(L, s!(b"what\x00"), ar.what);
        }
        if !strchr(options, 'l' as i32).is_null() {
            settabsi(L, s!(b"currentline\x00"), ar.currentline);
        }
        if !strchr(options, 'u' as i32).is_null() {
            settabsi(L, s!(b"nups\x00"), ar.nups as lua_int);
            settabsi(L, s!(b"nparams\x00"), ar.nparams as lua_int);
            settabsb(L, s!(b"isvararg\x00"), ar.isvararg as lua_int);
        }
        if !strchr(options, 'n' as i32).is_null() {
            settabss(L, s!(b"name\x00"), ar.name);
            settabss(L, s!(b"namewhat\x00"), ar.namewhat);
        }
        if !strchr(options, 't' as i32).is_null() {
            settabsb(L, s!(b"istailcall\x00"), ar.istailcall as lua_int);
        }
        if !strchr(options, 'L' as i32).is_null() {
            treatstackoption(L, L1, s!(b"activelines\x00"));
        }
        if !strchr(options, 'f' as i32).is_null() {
            treatstackoption(L, L1, s!(b"func\x00"));
        }
        /* return table */
        return 1i32;
    };
}
/*
** In function 'db_getinfo', the call to 'lua_getinfo' may push
** results on the stack; later it creates the result table to put
** these objects. Function 'treatstackoption' puts the result from
** 'lua_getinfo' on top of the result table so that it can call
** 'lua_setfield'.
*/
unsafe extern "C" fn treatstackoption(
    mut L: *mut lua_State,
    mut L1: *mut lua_State,
    mut fname: *const lua_char,
) -> () {
    if L == L1 {
        /* exchange object and table */
        lua_rotate(L, -2i32, 1i32);
    } else {
        /* move object to the "main" stack */
        lua_xmove(L1, L, 1i32);
    }
    /* put object into table */
    lua_setfield(L, -2i32, fname);
}
unsafe extern "C" fn settabsb(mut L: *mut lua_State, mut k: *const lua_char, mut v: lua_int) -> () {
    lua_pushboolean(L, v);
    lua_setfield(L, -2i32, k);
}
/*
** Variations of 'lua_settable', used by 'db_getinfo' to put results
** from 'lua_getinfo' into result table. Key is always a string;
** value can be a string, an int, or a boolean.
*/
unsafe extern "C" fn settabss(
    mut L: *mut lua_State,
    mut k: *const lua_char,
    mut v: *const lua_char,
) -> () {
    lua_pushstring(L, v);
    lua_setfield(L, -2i32, k);
}
unsafe extern "C" fn settabsi(mut L: *mut lua_State, mut k: *const lua_char, mut v: lua_int) -> () {
    lua_pushinteger(L, v as lua_Integer);
    lua_setfield(L, -2i32, k);
}
unsafe extern "C" fn db_gethook(mut L: *mut lua_State) -> lua_int {
    let mut arg: lua_int = 0;
    let mut L1: *mut lua_State = getthread(L, &mut arg);
    let mut buff: [lua_char; 5] = [0; 5];
    let mut mask: lua_int = lua_gethookmask(L1);
    let mut hook: lua_Hook = lua_gethook(L1);
    /* no hook? */
    if hook.is_none() {
        lua_pushnil(L);
    } else if hook != Some(hookf) {
        lua_pushstring(L, s!(b"external hook\x00"));
    } else {
        /* hook table must exist */
        lua_rawgetp(
            L,
            -1000000i32 - 1000i32,
            &HOOKKEY as *const lua_int as *const lua_void,
        );
        checkstack(L, L1, 1i32);
        lua_pushthread(L1);
        lua_xmove(L1, L, 1i32);
        /* 1st result = hooktable[L1] */
        lua_rawget(L, -2i32);
        /* remove hook table */
        lua_rotate(L, -2i32, -1i32);
        lua_settop(L, -1i32 - 1i32);
    }
    /* 2nd result = mask */
    lua_pushstring(L, unmakemask(mask, buff.as_mut_ptr()));
    /* 3rd result = count */
    lua_pushinteger(L, lua_gethookcount(L1) as lua_Integer);
    return 3i32;
}
/*
** Convert a bit mask (for 'gethook') into a string mask
*/
unsafe extern "C" fn unmakemask(mut mask: lua_int, mut smask: *mut lua_char) -> *mut lua_char {
    let mut i: lua_int = 0i32;
    if 0 != mask & 1i32 << 0i32 {
        let fresh0 = i;
        i = i + 1;
        *smask.offset(fresh0 as isize) = 'c' as i32 as lua_char
    }
    if 0 != mask & 1i32 << 1i32 {
        let fresh1 = i;
        i = i + 1;
        *smask.offset(fresh1 as isize) = 'r' as i32 as lua_char
    }
    if 0 != mask & 1i32 << 2i32 {
        let fresh2 = i;
        i = i + 1;
        *smask.offset(fresh2 as isize) = 'l' as i32 as lua_char
    }
    *smask.offset(i as isize) = '\u{0}' as i32 as lua_char;
    return smask;
}
unsafe extern "C" fn db_getuservalue(mut L: *mut lua_State) -> lua_int {
    if lua_type(L, 1i32) != 7i32 {
        lua_pushnil(L);
    } else {
        lua_getuservalue(L, 1i32);
    }
    return 1i32;
}
unsafe extern "C" fn db_debug(mut L: *mut lua_State) -> lua_int {
    loop {
        let mut buffer: [lua_char; 250] = [0; 250];
        fprintf(stderr, s!(b"%s\x00"), s!(b"lua_debug> \x00"));
        fflush(stderr);
        if fgets(
            buffer.as_mut_ptr(),
            ::std::mem::size_of::<[lua_char; 250]>() as lua_ulong as lua_int,
            stdin,
        )
        .is_null()
            || strcmp(buffer.as_mut_ptr(), s!(b"cont\n\x00")) == 0i32
        {
            return 0i32;
        } else {
            if 0 != luaL_loadbufferx(
                L,
                buffer.as_mut_ptr(),
                strlen(buffer.as_mut_ptr()),
                s!(b"=(debug command)\x00"),
                0 as *const lua_char,
            ) || 0 != lua_pcallk(L, 0i32, 0i32, 0i32, 0i32 as lua_KContext, None)
            {
                fprintf(
                    stderr,
                    s!(b"%s\n\x00"),
                    lua_tolstring(L, -1i32, 0 as *mut size_t),
                );
                fflush(stderr);
            }
            /* remove eventual returns */
            lua_settop(L, 0i32);
        }
    }
}
