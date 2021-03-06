use super::prelude::*;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct RN {
    pub f: *mut FILE,
    pub c: lua_int,
    pub n: lua_int,
    pub buff: [lua_char; 201],
}

#[no_mangle]
pub unsafe extern "C" fn luaopen_io(mut L: *mut lua_State) -> lua_int {
    /* new module */
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
    luaL_setfuncs(L, iolib.as_ptr(), 0i32);
    createmeta(L);
    /* create (and set) default files */
    createstdfile(L, stdin, s!(b"_IO_input\x00"), s!(b"stdin\x00"));
    createstdfile(L, stdout, s!(b"_IO_output\x00"), s!(b"stdout\x00"));
    createstdfile(L, stderr, 0 as *const lua_char, s!(b"stderr\x00"));
    return 1i32;
}
unsafe extern "C" fn createstdfile(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut k: *const lua_char,
    mut fname: *const lua_char,
) -> () {
    let mut p: *mut LStream = newprefile(L);
    (*p).f = f;
    (*p).closef = Some(io_noclose);
    if !k.is_null() {
        lua_pushvalue(L, -1i32);
        /* add file to registry */
        lua_setfield(L, -1000000i32 - 1000i32, k);
    }
    /* add file to module */
    lua_setfield(L, -2i32, fname);
}
/*
** function to (not) close the standard files stdin, stdout, and stderr
*/
unsafe extern "C" fn io_noclose(mut L: *mut lua_State) -> lua_int {
    let mut p: *mut LStream = luaL_checkudata(L, 1i32, s!(b"FILE*\x00")) as *mut LStream;
    /* keep file opened */
    (*p).closef = Some(io_noclose);
    lua_pushnil(L);
    lua_pushstring(L, s!(b"cannot close standard file\x00"));
    return 2i32;
}
/*
** When creating file handles, always creates a 'closed' file handle
** before opening the actual file; so, if there is a memory error, the
** handle is in a consistent state.
*/
unsafe extern "C" fn newprefile(mut L: *mut lua_State) -> *mut LStream {
    let mut p: *mut LStream =
        lua_newuserdata(L, ::std::mem::size_of::<LStream>() as lua_ulong) as *mut LStream;
    /* mark file handle as 'closed' */
    (*p).closef = None;
    luaL_setmetatable(L, s!(b"FILE*\x00"));
    return p;
}
unsafe extern "C" fn createmeta(mut L: *mut lua_State) -> () {
    /* create metatable for file handles */
    luaL_newmetatable(L, s!(b"FILE*\x00"));
    /* push metatable */
    lua_pushvalue(L, -1i32);
    /* metatable.__index = metatable */
    lua_setfield(L, -2i32, s!(b"__index\x00"));
    /* add file methods to new metatable */
    luaL_setfuncs(L, flib.as_ptr(), 0i32);
    /* pop new metatable */
    lua_settop(L, -1i32 - 1i32);
}
/*
** methods for file handles
*/
static mut flib: [luaL_Reg; 10] = [
    lua_reg!(b"close\x00", f_close),
    lua_reg!(b"flush\x00", f_flush),
    lua_reg!(b"lines\x00", f_lines),
    lua_reg!(b"read\x00", f_read),
    lua_reg!(b"seek\x00", f_seek),
    lua_reg!(b"setvbuf\x00", f_setvbuf),
    lua_reg!(b"write\x00", f_write),
    lua_reg!(b"__gc\x00", f_gc),
    lua_reg!(b"__tostring\x00", f_tostring),
    lua_reg_none!(0),
];
unsafe extern "C" fn f_tostring(mut L: *mut lua_State) -> lua_int {
    let mut p: *mut LStream = luaL_checkudata(L, 1i32, s!(b"FILE*\x00")) as *mut LStream;
    if (*p).closef.is_none() {
        lua_pushstring(L, s!(b"file (closed)\x00"));
    } else {
        lua_pushfstring!(L, s!(b"file (%p)\x00"), (*p).f,);
    }
    return 1i32;
}
unsafe extern "C" fn f_gc(mut L: *mut lua_State) -> lua_int {
    let mut p: *mut LStream = luaL_checkudata(L, 1i32, s!(b"FILE*\x00")) as *mut LStream;
    if (*p).closef.is_some() && !(*p).f.is_null() {
        /* ignore closed and incompletely open files */
        aux_close(L);
    }
    return 0i32;
}
/*
** Calls the 'close' function from a file handle. The 'volatile' avoids
** a bug in some versions of the Clang compiler (e.g., clang 3.0 for
** 32 bits).
*/
unsafe extern "C" fn aux_close(mut L: *mut lua_State) -> lua_int {
    let mut p: *mut LStream = luaL_checkudata(L, 1i32, s!(b"FILE*\x00")) as *mut LStream;
    let mut cf: lua_CFunction = (*p).closef;
    /* mark stream as closed */
    (*p).closef = None;
    /* close it */
    return cf.expect("non-null function pointer")(L);
}
unsafe extern "C" fn f_write(mut L: *mut lua_State) -> lua_int {
    let mut f: *mut FILE = tofile(L);
    /* push file at the stack top (to be returned) */
    lua_pushvalue(L, 1i32);
    return g_write(L, f, 2i32);
}
unsafe extern "C" fn tofile(mut L: *mut lua_State) -> *mut FILE {
    let mut p: *mut LStream = luaL_checkudata(L, 1i32, s!(b"FILE*\x00")) as *mut LStream;
    if (*p).closef.is_none() {
        luaL_error!(L, s!(b"attempt to use a closed file\x00"));
    }
    return (*p).f;
}
/* }====================================================== */
unsafe extern "C" fn g_write(mut L: *mut lua_State, mut f: *mut FILE, mut arg: lua_int) -> lua_int {
    let mut len: lua_int = 0;
    let mut nargs: lua_int = lua_gettop(L) - arg;
    let mut status: lua_int = 1i32;
    loop {
        let fresh1 = nargs;
        nargs = nargs - 1;
        if !(0 != fresh1) {
            break;
        }
        if lua_type(L, arg) == 3i32 {
            /* optimization: could be done exactly as for strings */
            len = if 0 != lua_isinteger(L, arg) {
                fprintf(
                    f,
                    s!(b"%lld\x00"),
                    lua_tointegerx(L, arg, 0 as *mut lua_int),
                )
            } else {
                fprintf(
                    f,
                    s!(b"%.14g\x00"),
                    lua_tonumberx(L, arg, 0 as *mut lua_int),
                )
            };
            status = (0 != status && len > 0i32) as lua_int
        } else {
            let mut l: size_t = 0;
            let mut s: *const lua_char = luaL_checklstring(L, arg, &mut l);
            status = (0 != status
                && fwrite(
                    s as *const lua_void,
                    ::std::mem::size_of::<lua_char>() as lua_ulong,
                    l,
                    f,
                ) == l) as lua_int
        }
        arg += 1
    }
    if 0 != status {
        /* file handle already on stack top */
        return 1i32;
    } else {
        return luaL_fileresult(L, status, 0 as *const lua_char);
    };
}
unsafe extern "C" fn f_setvbuf(mut L: *mut lua_State) -> lua_int {
    static mut mode: [lua_int; 3] = [2i32, 0i32, 1i32];
    static mut modenames: [*const lua_char; 4] = [
        s!(b"no\x00"),
        s!(b"full\x00"),
        s!(b"line\x00"),
        0 as *const lua_char,
    ];
    let mut f: *mut FILE = tofile(L);
    let mut op: lua_int = luaL_checkoption(L, 2i32, 0 as *const lua_char, modenames.as_ptr());
    let mut sz: lua_Integer = luaL_optinteger(
        L,
        3i32,
        (0x80i32 as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut lua_void>() as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_Integer>() as lua_ulong) as lua_int
            as lua_Integer,
    );
    let mut res: lua_int = setvbuf(f, 0 as *mut lua_char, mode[op as usize], sz as size_t);
    return luaL_fileresult(L, (res == 0i32) as lua_int, 0 as *const lua_char);
}
unsafe extern "C" fn f_seek(mut L: *mut lua_State) -> lua_int {
    static mut mode: [lua_int; 3] = [0i32, 1i32, 2i32];
    static mut modenames: [*const lua_char; 4] = [
        s!(b"set\x00"),
        s!(b"cur\x00"),
        s!(b"end\x00"),
        0 as *const lua_char,
    ];
    let mut f: *mut FILE = tofile(L);
    let mut op: lua_int = luaL_checkoption(L, 2i32, s!(b"cur\x00"), modenames.as_ptr());
    let mut p3: lua_Integer = luaL_optinteger(L, 3i32, 0i32 as lua_Integer);
    let mut offset: off_t = p3 as off_t;
    (offset as lua_Integer == p3
        || 0 != luaL_argerror(L, 3i32, s!(b"not an integer in proper range\x00"))) as lua_int;
    op = fseeko(f, offset, mode[op as usize]);
    if 0 != op {
        /* error */
        return luaL_fileresult(L, 0i32, 0 as *const lua_char);
    } else {
        lua_pushinteger(L, ftello(f) as lua_Integer);
        return 1i32;
    };
}
unsafe extern "C" fn f_read(mut L: *mut lua_State) -> lua_int {
    return g_read(L, tofile(L), 2i32);
}
unsafe extern "C" fn g_read(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut first: lua_int,
) -> lua_int {
    let mut l: size_t = 0;
    let mut nargs: lua_int = lua_gettop(L) - 1i32;
    let mut success: lua_int = 0;
    let mut n: lua_int = 0;
    clearerr(f);
    if nargs == 0i32 {
        /* no arguments? */
        success = read_line(L, f, 1i32);
        /* to return 1 result */
        n = first + 1i32
    } else {
        /* ensure stack space for all results and for auxlib's buffer */
        luaL_checkstack(L, nargs + 20i32, s!(b"too many arguments\x00"));
        success = 1i32;
        n = first;
        loop {
            let fresh2 = nargs;
            nargs = nargs - 1;
            if !(0 != fresh2 && 0 != success) {
                break;
            }
            if lua_type(L, n) == 3i32 {
                l = luaL_checkinteger(L, n) as size_t;
                success = if l == 0i32 as lua_ulong {
                    test_eof(L, f)
                } else {
                    read_chars(L, f, l)
                }
            } else {
                let mut p: *const lua_char = luaL_checklstring(L, n, 0 as *mut size_t);
                if *p as lua_int == '*' as i32 {
                    /* skip optional '*' (for compatibility) */
                    p = p.offset(1isize)
                }
                match *p as lua_int {
                    110 => success = read_number(L, f),
                    108 => success = read_line(L, f, 1i32),
                    76 => success = read_line(L, f, 0i32),
                    97 => {
                        /* read entire file */
                        read_all(L, f);
                        /* always success */
                        success = 1i32
                    }
                    _ => return luaL_argerror(L, n, s!(b"invalid format\x00")),
                }
            }
            n += 1
        }
    }
    if 0 != ferror(f) {
        return luaL_fileresult(L, 0i32, 0 as *const lua_char);
    } else {
        if 0 == success {
            /* remove last result */
            lua_settop(L, -1i32 - 1i32);
            /* push nil instead */
            lua_pushnil(L);
        }
        return n - first;
    };
}
unsafe extern "C" fn read_all(mut L: *mut lua_State, mut f: *mut FILE) -> () {
    let mut p: *mut lua_char = 0 as *mut lua_char;
    let mut nr: size_t = 0;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    loop {
        /* read file in chunks of LUAL_BUFFERSIZE bytes */
        p = luaL_prepbuffsize(
            &mut b,
            (0x80i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut lua_void>() as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_Integer>() as lua_ulong)
                as lua_int as size_t,
        );
        nr = fread(
            p as *mut lua_void,
            ::std::mem::size_of::<lua_char>() as lua_ulong,
            (0x80i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut lua_void>() as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_Integer>() as lua_ulong)
                as lua_int as size_t,
            f,
        );
        b.n = (b.n as lua_ulong).wrapping_add(nr) as size_t as size_t;
        if !(nr
            == (0x80i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut lua_void>() as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_Integer>() as lua_ulong)
                as lua_int as lua_ulong)
        {
            break;
        }
    }
    /* close buffer */
    luaL_pushresult(&mut b);
}
unsafe extern "C" fn read_line(
    mut L: *mut lua_State,
    mut f: *mut FILE,
    mut chop: lua_int,
) -> lua_int {
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    let mut c: lua_int = '\u{0}' as i32;
    luaL_buffinit(L, &mut b);
    while c != -1i32 && c != '\n' as i32 {
        /* repeat until end of line */
        /* preallocate buffer */
        let mut buff: *mut lua_char = luaL_prepbuffsize(
            &mut b,
            (0x80i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut lua_void>() as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_Integer>() as lua_ulong)
                as lua_int as size_t,
        );
        let mut i: lua_int = 0i32;
        /* no memory errors can happen inside the lock */
        flockfile(f);
        while i
            < (0x80i32 as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut lua_void>() as lua_ulong)
                .wrapping_mul(::std::mem::size_of::<lua_Integer>() as lua_ulong)
                as lua_int
            && {
                c = l_getc(f);
                c != -1i32
            }
            && c != '\n' as i32
        {
            let fresh3 = i;
            i = i + 1;
            *buff.offset(fresh3 as isize) = c as lua_char
        }
        funlockfile(f);
        b.n = (b.n as lua_ulong).wrapping_add(i as lua_ulong) as size_t as size_t
    }
    /* want a newline and have one? */
    if 0 == chop && c == '\n' as i32 {
        /* add ending newline to result */
        (b.n < b.size || !luaL_prepbuffsize(&mut b, 1i32 as size_t).is_null()) as lua_int;
        let fresh4 = b.n;
        b.n = b.n.wrapping_add(1);
        *b.b.offset(fresh4 as isize) = c as lua_char
    }
    /* close buffer */
    luaL_pushresult(&mut b);
    /* return ok if read something (either a newline or something else) */
    return (c == '\n' as i32 || lua_rawlen(L, -1i32) > 0i32 as lua_ulong) as lua_int;
}
/*
** Read a number: first reads a valid prefix of a numeral into a buffer.
** Then it calls 'lua_stringtonumber' to check whether the format is
** correct and to convert it to a Lua number
*/
unsafe extern "C" fn read_number(mut L: *mut lua_State, mut f: *mut FILE) -> lua_int {
    let mut rn: RN = RN {
        f: 0 as *mut FILE,
        c: 0,
        n: 0,
        buff: [0; 201],
    };
    let mut count: lua_int = 0i32;
    let mut hex: lua_int = 0i32;
    let mut decp: [lua_char; 2] = [0; 2];
    rn.f = f;
    rn.n = 0i32;
    /* get decimal point from locale */
    decp[0usize] = *(*localeconv()).decimal_point.offset(0isize);
    /* always accept a dot */
    decp[1usize] = '.' as i32 as lua_char;
    flockfile(rn.f);
    loop {
        /* skip spaces */
        rn.c = l_getc(rn.f);
        if !(0 != isspace(rn.c)) {
            break;
        }
    }
    /* optional signal */
    test2(&mut rn, s!(b"-+\x00"));
    if 0 != test2(&mut rn, s!(b"00\x00")) {
        if 0 != test2(&mut rn, s!(b"xX\x00")) {
            /* numeral is hexadecimal */
            hex = 1i32
        } else {
            count = 1i32
        }
    }
    /* integral part */
    count += readdigits(&mut rn, hex);
    /* decimal point? */
    if 0 != test2(&mut rn, decp.as_mut_ptr()) {
        /* fractional part */
        count += readdigits(&mut rn, hex)
    }
    if count > 0i32
        && 0 != test2(
            &mut rn,
            if 0 != hex {
                s!(b"pP\x00")
            } else {
                s!(b"eE\x00")
            },
        ) {
        /* exponent mark? */
        /* exponent signal */
        test2(&mut rn, s!(b"-+\x00"));
        /* exponent digits */
        readdigits(&mut rn, 0i32);
    }
    /* unread look-ahead char */
    ungetc(rn.c, rn.f);
    funlockfile(rn.f);
    /* finish string */
    rn.buff[rn.n as usize] = '\u{0}' as i32 as lua_char;
    /* is this a valid number? */
    if 0 != lua_stringtonumber(L, rn.buff.as_mut_ptr()) {
        /* ok */
        return 1i32;
    } else {
        /* invalid format */
        /* "result" to be removed */
        lua_pushnil(L);
        /* read fails */
        return 0i32;
    };
}
/*
** Read a sequence of (hex)digits
*/
unsafe extern "C" fn readdigits(mut rn: *mut RN, mut hex: lua_int) -> lua_int {
    let mut count: lua_int = 0i32;
    while 0
        != if 0 != hex {
            isxdigit((*rn).c)
        } else {
            isdigit((*rn).c)
        }
        && 0 != nextc(rn)
    {
        count += 1
    }
    return count;
}
/*
** Add current char to buffer (if not out of space) and read next one
*/
unsafe extern "C" fn nextc(mut rn: *mut RN) -> lua_int {
    if (*rn).n >= 200i32 {
        /* buffer overflow? */
        /* invalidate result */
        (*rn).buff[0usize] = '\u{0}' as i32 as lua_char;
        /* fail */
        return 0i32;
    } else {
        /* save current char */
        let fresh5 = (*rn).n;
        (*rn).n = (*rn).n + 1;
        (*rn).buff[fresh5 as usize] = (*rn).c as lua_char;
        /* read next one */
        (*rn).c = l_getc((*rn).f);
        return 1i32;
    };
}
/*
** Accept current char if it is in 'set' (of size 2)
*/
unsafe extern "C" fn test2(mut rn: *mut RN, mut set: *const lua_char) -> lua_int {
    if (*rn).c == *set.offset(0isize) as lua_int || (*rn).c == *set.offset(1isize) as lua_int {
        return nextc(rn);
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn read_chars(mut L: *mut lua_State, mut f: *mut FILE, mut n: size_t) -> lua_int {
    /* number of chars actually read */
    let mut nr: size_t = 0;
    let mut p: *mut lua_char = 0 as *mut lua_char;
    let mut b: luaL_Buffer = luaL_Buffer {
        b: 0 as *mut lua_char,
        size: 0,
        n: 0,
        L: 0 as *mut lua_State,
        initb: [0; 8192],
    };
    luaL_buffinit(L, &mut b);
    /* prepare buffer to read whole block */
    p = luaL_prepbuffsize(&mut b, n);
    /* try to read 'n' chars */
    nr = fread(
        p as *mut lua_void,
        ::std::mem::size_of::<lua_char>() as lua_ulong,
        n,
        f,
    );
    b.n = (b.n as lua_ulong).wrapping_add(nr) as size_t as size_t;
    /* close buffer */
    luaL_pushresult(&mut b);
    /* true iff read something */
    return (nr > 0i32 as lua_ulong) as lua_int;
}
unsafe extern "C" fn test_eof(mut L: *mut lua_State, mut f: *mut FILE) -> lua_int {
    let mut c: lua_int = getc(f);
    /* no-op when c == EOF */
    ungetc(c, f);
    lua_pushstring(L, s!(b"\x00"));
    return (c != -1i32) as lua_int;
}
unsafe extern "C" fn f_lines(mut L: *mut lua_State) -> lua_int {
    /* check that it's a valid file handle */
    tofile(L);
    aux_lines(L, 0i32);
    return 1i32;
}
/*
** maximum number of arguments to 'f:lines'/'io.lines' (it + 3 must fit
** in the limit for upvalues of a closure)
*/
unsafe extern "C" fn aux_lines(mut L: *mut lua_State, mut toclose: lua_int) -> () {
    /* number of arguments to read */
    let mut n: lua_int = lua_gettop(L) - 1i32;
    (n <= 250i32 || 0 != luaL_argerror(L, 250i32 + 2i32, s!(b"too many arguments\x00"))) as lua_int;
    /* number of arguments to read */
    lua_pushinteger(L, n as lua_Integer);
    /* close/not close file when finished */
    lua_pushboolean(L, toclose);
    /* move 'n' and 'toclose' to their positions */
    lua_rotate(L, 2i32, 2i32);
    lua_pushcclosure(L, Some(io_readline), 3i32 + n);
}
unsafe extern "C" fn io_readline(mut L: *mut lua_State) -> lua_int {
    let mut p: *mut LStream = lua_touserdata(L, -1000000i32 - 1000i32 - 1i32) as *mut LStream;
    let mut i: lua_int = 0;
    let mut n: lua_int =
        lua_tointegerx(L, -1000000i32 - 1000i32 - 2i32, 0 as *mut lua_int) as lua_int;
    /* file is already closed? */
    if (*p).closef.is_none() {
        return luaL_error!(L, s!(b"file is already closed\x00"));
    } else {
        lua_settop(L, 1i32);
        luaL_checkstack(L, n, s!(b"too many arguments\x00"));
        /* push arguments to 'g_read' */
        i = 1i32;
        while i <= n {
            lua_pushvalue(L, -1000000i32 - 1000i32 - (3i32 + i));
            i += 1
        }
        /* 'n' is number of results */
        n = g_read(L, (*p).f, 2i32);
        /* should return at least a nil */
        /* read at least one value? */
        if 0 != lua_toboolean(L, -n) {
            /* return them */
            return n;
        } else if n > 1i32 {
            /* is there error information? */
            /* 2nd result is error message */
            return luaL_error!(
                L,
                s!(b"%s\x00"),
                lua_tolstring(L, -n + 1i32, 0 as *mut size_t),
            );
        } else {
            if 0 != lua_toboolean(L, -1000000i32 - 1000i32 - 3i32) {
                /* generator created file? */
                lua_settop(L, 0i32);
                lua_pushvalue(L, -1000000i32 - 1000i32 - 1i32);
                /* close it */
                aux_close(L);
            }
            return 0i32;
        }
    };
}
unsafe extern "C" fn f_flush(mut L: *mut lua_State) -> lua_int {
    return luaL_fileresult(
        L,
        (fflush(tofile(L)) == 0i32) as lua_int,
        0 as *const lua_char,
    );
}
unsafe extern "C" fn f_close(mut L: *mut lua_State) -> lua_int {
    /* make sure argument is an open stream */
    tofile(L);
    return aux_close(L);
}
/*
** functions for 'io' library
*/
static mut iolib: [luaL_Reg; 12] = [
    lua_reg!(b"close\x00", io_close),
    lua_reg!(b"flush\x00", io_flush),
    lua_reg!(b"input\x00", io_input),
    lua_reg!(b"lines\x00", io_lines),
    lua_reg!(b"open\x00", io_open),
    lua_reg!(b"output\x00", io_output),
    lua_reg!(b"popen\x00", io_popen),
    lua_reg!(b"read\x00", io_read),
    lua_reg!(b"tmpfile\x00", io_tmpfile),
    lua_reg!(b"type\x00", io_type),
    lua_reg!(b"write\x00", io_write),
    lua_reg_none!(0),
];
unsafe extern "C" fn io_write(mut L: *mut lua_State) -> lua_int {
    return g_write(L, getiofile(L, s!(b"_IO_output\x00")), 1i32);
}
unsafe extern "C" fn getiofile(mut L: *mut lua_State, mut findex: *const lua_char) -> *mut FILE {
    let mut p: *mut LStream = 0 as *mut LStream;
    lua_getfield(L, -1000000i32 - 1000i32, findex);
    p = lua_touserdata(L, -1i32) as *mut LStream;
    if (*p).closef.is_none() {
        luaL_error!(
            L,
            s!(b"standard %s file is closed\x00"),
            findex.offset(
                (::std::mem::size_of::<[lua_char; 5]>() as lua_ulong)
                    .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                    .wrapping_sub(1i32 as lua_ulong) as isize,
            ),
        );
    }
    return (*p).f;
}
unsafe extern "C" fn io_type(mut L: *mut lua_State) -> lua_int {
    let mut p: *mut LStream = 0 as *mut LStream;
    luaL_checkany(L, 1i32);
    p = luaL_testudata(L, 1i32, s!(b"FILE*\x00")) as *mut LStream;
    if p.is_null() {
        /* not a file */
        lua_pushnil(L);
    } else if (*p).closef.is_none() {
        lua_pushstring(L, s!(b"closed file\x00"));
    } else {
        lua_pushstring(L, s!(b"file\x00"));
    }
    return 1i32;
}
unsafe extern "C" fn io_tmpfile(mut L: *mut lua_State) -> lua_int {
    let mut p: *mut LStream = newfile(L);
    (*p).f = tmpfile();
    return if (*p).f.is_null() {
        luaL_fileresult(L, 0i32, 0 as *const lua_char)
    } else {
        1i32
    };
}
unsafe extern "C" fn newfile(mut L: *mut lua_State) -> *mut LStream {
    let mut p: *mut LStream = newprefile(L);
    (*p).f = 0 as *mut FILE;
    (*p).closef = Some(io_fclose);
    return p;
}
/*
** function to close regular files
*/
unsafe extern "C" fn io_fclose(mut L: *mut lua_State) -> lua_int {
    let mut p: *mut LStream = luaL_checkudata(L, 1i32, s!(b"FILE*\x00")) as *mut LStream;
    let mut res: lua_int = fclose((*p).f);
    return luaL_fileresult(L, (res == 0i32) as lua_int, 0 as *const lua_char);
}
unsafe extern "C" fn io_read(mut L: *mut lua_State) -> lua_int {
    return g_read(L, getiofile(L, s!(b"_IO_input\x00")), 1i32);
}
unsafe extern "C" fn io_popen(mut L: *mut lua_State) -> lua_int {
    let mut filename: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut mode: *const lua_char = luaL_optlstring(L, 2i32, s!(b"r\x00"), 0 as *mut size_t);
    let mut p: *mut LStream = newprefile(L);
    fflush(0 as *mut FILE);
    (*p).f = popen(filename, mode);
    (*p).closef = Some(io_pclose);
    return if (*p).f.is_null() {
        luaL_fileresult(L, 0i32, filename)
    } else {
        1i32
    };
}
/*
** function to close 'popen' files
*/
unsafe extern "C" fn io_pclose(mut L: *mut lua_State) -> lua_int {
    let mut p: *mut LStream = luaL_checkudata(L, 1i32, s!(b"FILE*\x00")) as *mut LStream;
    return luaL_execresult(L, pclose((*p).f));
}
unsafe extern "C" fn io_output(mut L: *mut lua_State) -> lua_int {
    return g_iofile(L, s!(b"_IO_output\x00"), s!(b"w\x00"));
}
unsafe extern "C" fn g_iofile(
    mut L: *mut lua_State,
    mut f: *const lua_char,
    mut mode: *const lua_char,
) -> lua_int {
    if !(lua_type(L, 1i32) <= 0i32) {
        let mut filename: *const lua_char = lua_tolstring(L, 1i32, 0 as *mut size_t);
        if !filename.is_null() {
            opencheck(L, filename, mode);
        } else {
            /* check that it's a valid file handle */
            tofile(L);
            lua_pushvalue(L, 1i32);
        }
        lua_setfield(L, -1000000i32 - 1000i32, f);
    }
    /* return current value */
    lua_getfield(L, -1000000i32 - 1000i32, f);
    return 1i32;
}
unsafe extern "C" fn opencheck(
    mut L: *mut lua_State,
    mut fname: *const lua_char,
    mut mode: *const lua_char,
) -> () {
    let mut p: *mut LStream = newfile(L);
    (*p).f = fopen(fname, mode);
    if (*p).f.is_null() {
        luaL_error!(
            L,
            s!(b"cannot open file \'%s\' (%s)\x00"),
            fname,
            strerror(errno),
        );
    };
}
unsafe extern "C" fn io_open(mut L: *mut lua_State) -> lua_int {
    let mut filename: *const lua_char = luaL_checklstring(L, 1i32, 0 as *mut size_t);
    let mut mode: *const lua_char = luaL_optlstring(L, 2i32, s!(b"r\x00"), 0 as *mut size_t);
    let mut p: *mut LStream = newfile(L);
    /* to traverse/check mode */
    let mut md: *const lua_char = mode;
    (0 != l_checkmode(md) || 0 != luaL_argerror(L, 2i32, s!(b"invalid mode\x00"))) as lua_int;
    (*p).f = fopen(filename, mode);
    return if (*p).f.is_null() {
        luaL_fileresult(L, 0i32, filename)
    } else {
        1i32
    };
}
/*
** $Id: liolib.c,v 2.151.1.1 2017/04/19 17:29:57 roberto Exp $
** Standard I/O (and system) library
** See Copyright Notice in lua.h
*/
/*
** Change this macro to accept other modes for 'fopen' besides
** the standard ones.
*/
/* accepted extensions to 'mode' in 'fopen' */
/* Check whether 'mode' matches '[rwa]%+?[L_MODEEXT]*' */
unsafe extern "C" fn l_checkmode(mut mode: *const lua_char) -> lua_int {
    return (*mode as lua_int != '\u{0}' as i32
        && {
            let fresh6 = mode;
            mode = mode.offset(1);
            !strchr(s!(b"rwa\x00"), *fresh6 as lua_int).is_null()
        }
        && (*mode as lua_int != '+' as i32 || {
            mode = mode.offset(1isize);
            0 != 1i32
        })
        && strspn(mode, s!(b"b\x00")) == strlen(mode)) as lua_int;
}
unsafe extern "C" fn io_lines(mut L: *mut lua_State) -> lua_int {
    let mut filename: *const lua_char = 0 as *const lua_char;
    let mut toclose: lua_int = 0;
    if lua_type(L, 1i32) == -1i32 {
        /* at least one argument */
        lua_pushnil(L);
    }
    if lua_type(L, 1i32) == 0i32 {
        /* no file name? */
        /* get default input */
        lua_getfield(L, -1000000i32 - 1000i32, s!(b"_IO_input\x00"));
        /* put it at index 1 */
        lua_copy(L, -1i32, 1i32);
        lua_settop(L, -1i32 - 1i32);
        /* check that it's a valid file handle */
        tofile(L);
        /* do not close it after iteration */
        toclose = 0i32
    } else {
        /* open a new file */
        filename = luaL_checklstring(L, 1i32, 0 as *mut size_t);
        opencheck(L, filename, s!(b"r\x00"));
        /* put file at index 1 */
        lua_copy(L, -1i32, 1i32);
        lua_settop(L, -1i32 - 1i32);
        /* close it after iteration */
        toclose = 1i32
    }
    aux_lines(L, toclose);
    return 1i32;
}
unsafe extern "C" fn io_input(mut L: *mut lua_State) -> lua_int {
    return g_iofile(L, s!(b"_IO_input\x00"), s!(b"r\x00"));
}
unsafe extern "C" fn io_flush(mut L: *mut lua_State) -> lua_int {
    return luaL_fileresult(
        L,
        (fflush(getiofile(L, s!(b"_IO_output\x00"))) == 0i32) as lua_int,
        0 as *const lua_char,
    );
}
unsafe extern "C" fn io_close(mut L: *mut lua_State) -> lua_int {
    /* no argument? */
    if lua_type(L, 1i32) == -1i32 {
        /* use standard output */
        lua_getfield(L, -1000000i32 - 1000i32, s!(b"_IO_output\x00"));
    }
    return f_close(L);
}
