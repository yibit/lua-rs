use super::prelude::*;

/*
* WARNING: if you change the order of this enumeration,
* grep "ORDER RESERVED"
*/
pub type RESERVED = lua_uint;
pub const TK_STRING: RESERVED = 293;
pub const TK_NAME: RESERVED = 292;
pub const TK_INT: RESERVED = 291;
pub const TK_FLT: RESERVED = 290;
pub const TK_EOS: RESERVED = 289;
pub const TK_DBCOLON: RESERVED = 288;
pub const TK_SHR: RESERVED = 287;
pub const TK_SHL: RESERVED = 286;
pub const TK_NE: RESERVED = 285;
pub const TK_LE: RESERVED = 284;
pub const TK_GE: RESERVED = 283;
pub const TK_EQ: RESERVED = 282;
pub const TK_DOTS: RESERVED = 281;
pub const TK_CONCAT: RESERVED = 280;
/* other terminal symbols */
pub const TK_IDIV: RESERVED = 279;
pub const TK_WHILE: RESERVED = 278;
pub const TK_UNTIL: RESERVED = 277;
pub const TK_TRUE: RESERVED = 276;
pub const TK_THEN: RESERVED = 275;
pub const TK_RETURN: RESERVED = 274;
pub const TK_REPEAT: RESERVED = 273;
pub const TK_OR: RESERVED = 272;
pub const TK_NOT: RESERVED = 271;
pub const TK_NIL: RESERVED = 270;
pub const TK_LOCAL: RESERVED = 269;
pub const TK_IN: RESERVED = 268;
pub const TK_IF: RESERVED = 267;
pub const TK_GOTO: RESERVED = 266;
pub const TK_FUNCTION: RESERVED = 265;
pub const TK_FOR: RESERVED = 264;
pub const TK_FALSE: RESERVED = 263;
pub const TK_END: RESERVED = 262;
pub const TK_ELSEIF: RESERVED = 261;
pub const TK_ELSE: RESERVED = 260;
pub const TK_DO: RESERVED = 259;
pub const TK_BREAK: RESERVED = 258;
/* terminal symbols denoted by reserved words */
pub const TK_AND: RESERVED = 257;
/* number of reserved words */

/*
** grep "ORDER OP" if you change these enums
*/
pub type OpCode = lua_uint;
/*	Ax	extra (larger) argument for previous opcode	*/
pub const OP_EXTRAARG: OpCode = 46;
/*	A B	R(A), R(A+1), ..., R(A+B-2) = vararg		*/
pub const OP_VARARG: OpCode = 45;
/*	A Bx	R(A) := closure(KPROTO[Bx])			*/
pub const OP_CLOSURE: OpCode = 44;
/*	A B C	R(A)[(C-1)*FPF+i] := R(A+i), 1 <= i <= B	*/
pub const OP_SETLIST: OpCode = 43;
/*	A sBx	if R(A+1) ~= nil then { R(A)=R(A+1); pc += sBx }*/
pub const OP_TFORLOOP: OpCode = 42;
/*	A C	R(A+3), ... ,R(A+2+C) := R(A)(R(A+1), R(A+2));	*/
pub const OP_TFORCALL: OpCode = 41;
/*	A sBx	R(A)-=R(A+2); pc+=sBx				*/
pub const OP_FORPREP: OpCode = 40;
/*	A sBx	R(A)+=R(A+2);
?= R(A+1) then { pc+=sBx; R(A+3)=R(A) }*/
pub const OP_FORLOOP: OpCode = 39;
/*	A B	return R(A), ... ,R(A+B-2)	(see note)	*/
pub const OP_RETURN: OpCode = 38;
/*	A B C	return R(A)(R(A+1), ... ,R(A+B-1))		*/
pub const OP_TAILCALL: OpCode = 37;
/*	A B C	R(A), ... ,R(A+C-2) := R(A)(R(A+1), ... ,R(A+B-1)) */
pub const OP_CALL: OpCode = 36;
/*	A B C	if (R(B) <=> C) then R(A) := R(B) else pc++	*/
pub const OP_TESTSET: OpCode = 35;
/*	A C	if not (R(A) <=> C) then pc++			*/
pub const OP_TEST: OpCode = 34;
/*	A B C	if ((RK(B) <= RK(C)) ~= A) then pc++		*/
pub const OP_LE: OpCode = 33;
/*	A B C	if ((RK(B) <  RK(C)) ~= A) then pc++		*/
pub const OP_LT: OpCode = 32;
/*	A B C	if ((RK(B) == RK(C)) ~= A) then pc++		*/
pub const OP_EQ: OpCode = 31;
/*	A sBx	pc+=sBx; if (A) close all upvalues >= R(A - 1)	*/
pub const OP_JMP: OpCode = 30;
/*	A B C	R(A) := R(B).. ... ..R(C)			*/
pub const OP_CONCAT: OpCode = 29;
/*	A B	R(A) := length of R(B)				*/
pub const OP_LEN: OpCode = 28;
/*	A B	R(A) := not R(B)				*/
pub const OP_NOT: OpCode = 27;
/*	A B	R(A) := ~R(B)					*/
pub const OP_BNOT: OpCode = 26;
/*	A B	R(A) := -R(B)					*/
pub const OP_UNM: OpCode = 25;
/*	A B C	R(A) := RK(B) >> RK(C)				*/
pub const OP_SHR: OpCode = 24;
/*	A B C	R(A) := RK(B) << RK(C)				*/
pub const OP_SHL: OpCode = 23;
/*	A B C	R(A) := RK(B) ~ RK(C)				*/
pub const OP_BXOR: OpCode = 22;
/*	A B C	R(A) := RK(B) | RK(C)				*/
pub const OP_BOR: OpCode = 21;
/*	A B C	R(A) := RK(B) & RK(C)				*/
pub const OP_BAND: OpCode = 20;
/*	A B C	R(A) := RK(B) // RK(C)				*/
pub const OP_IDIV: OpCode = 19;
/*	A B C	R(A) := RK(B) / RK(C)				*/
pub const OP_DIV: OpCode = 18;
/*	A B C	R(A) := RK(B) ^ RK(C)				*/
pub const OP_POW: OpCode = 17;
/*	A B C	R(A) := RK(B) % RK(C)				*/
pub const OP_MOD: OpCode = 16;
/*	A B C	R(A) := RK(B) * RK(C)				*/
pub const OP_MUL: OpCode = 15;
/*	A B C	R(A) := RK(B) - RK(C)				*/
pub const OP_SUB: OpCode = 14;
/*	A B C	R(A) := RK(B) + RK(C)				*/
pub const OP_ADD: OpCode = 13;
/*	A B C	R(A+1) := R(B); R(A) := R(B)[RK(C)]		*/
pub const OP_SELF: OpCode = 12;
/*	A B C	R(A) := {} (size = B,C)				*/
pub const OP_NEWTABLE: OpCode = 11;
/*	A B C	R(A)[RK(B)] := RK(C)				*/
pub const OP_SETTABLE: OpCode = 10;
/*	A B	UpValue[B] := R(A)				*/
pub const OP_SETUPVAL: OpCode = 9;
/*	A B C	UpValue[A][RK(B)] := RK(C)			*/
pub const OP_SETTABUP: OpCode = 8;
/*	A B C	R(A) := R(B)[RK(C)]				*/
pub const OP_GETTABLE: OpCode = 7;
/*	A B C	R(A) := UpValue[B][RK(C)]			*/
pub const OP_GETTABUP: OpCode = 6;
/*	A B	R(A) := UpValue[B]				*/
pub const OP_GETUPVAL: OpCode = 5;
/*	A B	R(A), R(A+1), ..., R(A+B) := nil		*/
pub const OP_LOADNIL: OpCode = 4;
/*	A B C	R(A) := (Bool)B; if (C) pc++			*/
pub const OP_LOADBOOL: OpCode = 3;
/*	A 	R(A) := Kst(extra arg)				*/
pub const OP_LOADKX: OpCode = 2;
/*	A Bx	R(A) := Kst(Bx)					*/
pub const OP_LOADK: OpCode = 1;
/*----------------------------------------------------------------------
name		args	description
------------------------------------------------------------------------*/
/*	A B	R(A) := R(B)					*/
pub const OP_MOVE: OpCode = 0;
/*
** $Id: lparser.h,v 1.76.1.1 2017/04/19 17:20:42 roberto Exp $
** Lua Parser
** See Copyright Notice in lua.h
*/
/*
** Expression and variable descriptor.
** Code generation for variables and expressions can be delayed to allow
** optimizations; An 'expdesc' structure describes a potentially-delayed
** variable/expression. It has a description of its "main" value plus a
** list of conditional jumps that can also produce its value (generated
** by short-circuit operators 'and'/'or').
*/

/*
** structure to chain all variables in the left-hand side of an
** assignment
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LHS_assign {
    pub prev: *mut LHS_assign,
    pub v: expdesc,
}

/*
** grep "ORDER OPR" if you change these enums  (ORDER OP)
*/
pub type BinOpr = lua_uint;
pub const OPR_NOBINOPR: BinOpr = 21;
pub const OPR_OR: BinOpr = 20;
pub const OPR_AND: BinOpr = 19;
pub const OPR_GE: BinOpr = 18;
pub const OPR_GT: BinOpr = 17;
pub const OPR_NE: BinOpr = 16;
pub const OPR_LE: BinOpr = 15;
pub const OPR_LT: BinOpr = 14;
pub const OPR_EQ: BinOpr = 13;
pub const OPR_CONCAT: BinOpr = 12;
pub const OPR_SHR: BinOpr = 11;
pub const OPR_SHL: BinOpr = 10;
pub const OPR_BXOR: BinOpr = 9;
pub const OPR_BOR: BinOpr = 8;
pub const OPR_BAND: BinOpr = 7;
pub const OPR_IDIV: BinOpr = 6;
pub const OPR_DIV: BinOpr = 5;
pub const OPR_POW: BinOpr = 4;
pub const OPR_MOD: BinOpr = 3;
pub const OPR_MUL: BinOpr = 2;
pub const OPR_SUB: BinOpr = 1;
pub const OPR_ADD: BinOpr = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_9 {
    pub left: lu_byte,
    pub right: lu_byte,
}
/*
** {======================================================================
** Rules for Constructors
** =======================================================================
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConsControl {
    pub v: expdesc,
    pub t: *mut expdesc,
    pub nh: lua_int,
    pub na: lua_int,
    pub tostore: lua_int,
}

pub type UnOpr = lua_uint;
pub const OPR_NOUNOPR: UnOpr = 4;
pub const OPR_LEN: UnOpr = 3;
pub const OPR_NOT: UnOpr = 2;
pub const OPR_BNOT: UnOpr = 1;
pub const OPR_MINUS: UnOpr = 0;
#[no_mangle]
pub unsafe extern "C" fn luaY_parser(
    mut L: *mut lua_State,
    mut z: *mut ZIO,
    mut buff: *mut Mbuffer,
    mut dyd: *mut Dyndata,
    mut name: *const lua_char,
    mut firstchar: lua_int,
) -> *mut LClosure {
    let mut lexstate: LexState = LexState {
        current: 0,
        linenumber: 0,
        lastline: 0,
        t: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        lookahead: Token {
            token: 0,
            seminfo: SemInfo { r: 0. },
        },
        fs: 0 as *mut FuncState,
        L: 0 as *mut lua_State,
        z: 0 as *mut ZIO,
        buff: 0 as *mut Mbuffer,
        h: 0 as *mut Table,
        dyd: 0 as *mut Dyndata,
        source: 0 as *mut TString,
        envn: 0 as *mut TString,
    };
    let mut funcstate: FuncState = FuncState {
        f: 0 as *mut Proto,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        jpc: 0,
        nk: 0,
        np: 0,
        firstlocal: 0,
        nlocvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
    };
    /* create main closure */
    let mut cl: *mut LClosure = luaF_newLclosure(L, 1i32);
    let mut io: *mut TValue = (*L).top;
    /* anchor it (to avoid being collected) */
    let mut x_: *mut LClosure = cl;
    (*io).value_.gc = &mut (*(x_ as *mut GCUnion)).gc;
    (*io).tt_ = 6i32 | 0i32 << 4i32 | 1i32 << 6i32;
    luaD_inctop(L);
    /* create table for scanner */
    lexstate.h = luaH_new(L);
    let mut io_0: *mut TValue = (*L).top;
    /* anchor it */
    let mut x__0: *mut Table = lexstate.h;
    (*io_0).value_.gc = &mut (*(x__0 as *mut GCUnion)).gc;
    (*io_0).tt_ = 5i32 | 1i32 << 6i32;
    luaD_inctop(L);
    (*cl).p = luaF_newproto(L);
    funcstate.f = (*cl).p;
    /* create and anchor TString */
    (*funcstate.f).source = luaS_new(L, name);
    /* do not need barrier here */
    lexstate.buff = buff;
    lexstate.dyd = dyd;
    (*dyd).label.n = 0i32;
    (*dyd).gt.n = (*dyd).label.n;
    (*dyd).actvar.n = (*dyd).gt.n;
    luaX_setinput(L, &mut lexstate, z, (*funcstate.f).source, firstchar);
    mainfunc(&mut lexstate, &mut funcstate);
    /* all scopes should be correctly finished */
    /* remove scanner's table */
    (*L).top = (*L).top.offset(-1isize);
    /* closure is on the stack, too */
    return cl;
}
/* }====================================================================== */
/*
** compiles the main function, which is a regular vararg function with an
** upvalue named LUA_ENV
*/
unsafe extern "C" fn mainfunc(mut ls: *mut LexState, mut fs: *mut FuncState) -> () {
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut v: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    open_func(ls, fs, &mut bl);
    /* main function is always declared vararg */
    (*(*fs).f).is_vararg = 1i32 as lu_byte;
    /* create and... */
    init_exp(&mut v, VLOCAL, 0i32);
    /* ...set environment upvalue */
    newupvalue(fs, (*ls).envn, &mut v);
    /* read first token */
    luaX_next(ls);
    /* parse main body */
    statlist(ls);
    check(ls, TK_EOS as lua_int);
    close_func(ls);
}
unsafe extern "C" fn close_func(mut ls: *mut LexState) -> () {
    let mut L: *mut lua_State = (*ls).L;
    let mut fs: *mut FuncState = (*ls).fs;
    let mut f: *mut Proto = (*fs).f;
    /* final return */
    luaK_ret(fs, 0i32, 0i32);
    leaveblock(fs);
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && ((*fs).pc as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<Instruction>() as lua_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).code = luaM_realloc_(
        L,
        (*f).code as *mut lua_void,
        ((*f).sizecode as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<Instruction>() as lua_ulong),
        ((*fs).pc as lua_ulong).wrapping_mul(::std::mem::size_of::<Instruction>() as lua_ulong),
    ) as *mut Instruction;
    (*f).sizecode = (*fs).pc;
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && ((*fs).pc as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<lua_int>() as lua_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).lineinfo = luaM_realloc_(
        L,
        (*f).lineinfo as *mut lua_void,
        ((*f).sizelineinfo as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<lua_int>() as lua_ulong),
        ((*fs).pc as lua_ulong).wrapping_mul(::std::mem::size_of::<lua_int>() as lua_ulong),
    ) as *mut lua_int;
    (*f).sizelineinfo = (*fs).pc;
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && ((*fs).nk as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<TValue>() as lua_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).k = luaM_realloc_(
        L,
        (*f).k as *mut lua_void,
        ((*f).sizek as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
        ((*fs).nk as lua_ulong).wrapping_mul(::std::mem::size_of::<TValue>() as lua_ulong),
    ) as *mut TValue;
    (*f).sizek = (*fs).nk;
    if ::std::mem::size_of::<lua_int>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && ((*fs).np as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<*mut Proto>() as lua_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).p = luaM_realloc_(
        L,
        (*f).p as *mut lua_void,
        ((*f).sizep as lua_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>() as lua_ulong),
        ((*fs).np as lua_ulong).wrapping_mul(::std::mem::size_of::<*mut Proto>() as lua_ulong),
    ) as *mut *mut Proto;
    (*f).sizep = (*fs).np;
    if ::std::mem::size_of::<lua_short>() as lua_ulong
        >= ::std::mem::size_of::<size_t>() as lua_ulong
        && ((*fs).nlocvars as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<LocVar>() as lua_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).locvars = luaM_realloc_(
        L,
        (*f).locvars as *mut lua_void,
        ((*f).sizelocvars as lua_ulong).wrapping_mul(::std::mem::size_of::<LocVar>() as lua_ulong),
        ((*fs).nlocvars as lua_ulong).wrapping_mul(::std::mem::size_of::<LocVar>() as lua_ulong),
    ) as *mut LocVar;
    (*f).sizelocvars = (*fs).nlocvars as lua_int;
    if ::std::mem::size_of::<lu_byte>() as lua_ulong >= ::std::mem::size_of::<size_t>() as lua_ulong
        && ((*fs).nups as size_t).wrapping_add(1i32 as lua_ulong)
            > (!(0i32 as size_t)).wrapping_div(::std::mem::size_of::<Upvaldesc>() as lua_ulong)
    {
        luaM_toobig(L);
    } else {
    };
    (*f).upvalues = luaM_realloc_(
        L,
        (*f).upvalues as *mut lua_void,
        ((*f).sizeupvalues as lua_ulong)
            .wrapping_mul(::std::mem::size_of::<Upvaldesc>() as lua_ulong),
        ((*fs).nups as lua_ulong).wrapping_mul(::std::mem::size_of::<Upvaldesc>() as lua_ulong),
    ) as *mut Upvaldesc;
    (*f).sizeupvalues = (*fs).nups as lua_int;
    (*ls).fs = (*fs).prev;
    if (*(*L).l_G).GCdebt > 0i32 as lua_long {
        luaC_step(L);
    };
}
unsafe extern "C" fn leaveblock(mut fs: *mut FuncState) -> () {
    let mut j: lua_int = 0;
    let mut bl: *mut BlockCnt = (*fs).bl;
    let mut ls: *mut LexState = (*fs).ls;
    if !(*bl).previous.is_null() && 0 != (*bl).upval as lua_int {
        /* create a 'jump to here' to close upvalues */
        j = luaK_jump(fs);
        luaK_patchclose(fs, j, (*bl).nactvar as lua_int);
        luaK_patchtohere(fs, j);
    }
    if 0 != (*bl).isloop {
        /* close pending breaks */
        breaklabel(ls);
    }
    (*fs).bl = (*bl).previous;
    removevars(fs, (*bl).nactvar as lua_int);
    /* free registers */
    (*fs).freereg = (*fs).nactvar;
    /* remove local labels */
    (*(*ls).dyd).label.n = (*bl).firstlabel;
    /* inner block? */
    if !(*bl).previous.is_null() {
        /* update pending gotos to outer block */
        movegotosout(fs, bl);
    } else if (*bl).firstgoto < (*(*ls).dyd).gt.n {
        /* error */
        undefgoto(
            ls,
            &mut *(*(*ls).dyd).gt.arr.offset((*bl).firstgoto as isize),
        );
    };
}
/*
** generates an error for an undefined 'goto'; choose appropriate
** message when label name is a reserved word (which can only be 'break')
*/
unsafe extern "C" fn undefgoto(mut ls: *mut LexState, mut gt: *mut Labeldesc) -> ! {
    let mut msg: *const lua_char = if (*(*gt).name).tt as lua_int == 4i32 | 0i32 << 4i32
        && (*(*gt).name).extra as lua_int > 0i32
    {
        s!(b"<%s> at line %d not inside a loop\x00")
    } else {
        s!(b"no visible label \'%s\' for <goto> at line %d\x00")
    };
    msg = luaO_pushfstring!(
        (*ls).L,
        msg,
        ((*gt).name as *mut lua_char)
            .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
        (*gt).line,
    );
    semerror(ls, msg);
}
/* semantic error */
unsafe extern "C" fn semerror(mut ls: *mut LexState, mut msg: *const lua_char) -> ! {
    /* remove "near <token>" from final message */
    (*ls).t.token = 0i32;
    luaX_syntaxerror(ls, msg);
}
/*
** export pending gotos to outer level, to check them against
** outer labels; if the block being exited has upvalues, and
** the goto exits the scope of any variable (which can be the
** upvalue), close those variables being exited.
*/
unsafe extern "C" fn movegotosout(mut fs: *mut FuncState, mut bl: *mut BlockCnt) -> () {
    let mut i: lua_int = (*bl).firstgoto;
    let mut gl: *mut Labellist = &mut (*(*(*fs).ls).dyd).gt;
    /* correct pending gotos to current block and try to close it
    with visible labels */
    while i < (*gl).n {
        let mut gt: *mut Labeldesc = &mut *(*gl).arr.offset(i as isize) as *mut Labeldesc;
        if (*gt).nactvar as lua_int > (*bl).nactvar as lua_int {
            if 0 != (*bl).upval {
                luaK_patchclose(fs, (*gt).pc, (*bl).nactvar as lua_int);
            }
            (*gt).nactvar = (*bl).nactvar
        }
        if !(0 == findlabel((*fs).ls, i)) {
            continue;
        }
        /* move to next one */
        i += 1
    }
}
/*
** try to close a goto with existing labels; this solves backward jumps
*/
unsafe extern "C" fn findlabel(mut ls: *mut LexState, mut g: lua_int) -> lua_int {
    let mut i: lua_int = 0;
    let mut bl: *mut BlockCnt = (*(*ls).fs).bl;
    let mut dyd: *mut Dyndata = (*ls).dyd;
    let mut gt: *mut Labeldesc = &mut *(*dyd).gt.arr.offset(g as isize) as *mut Labeldesc;
    /* check labels in current block for a match */
    i = (*bl).firstlabel;
    while i < (*dyd).label.n {
        let mut lb: *mut Labeldesc = &mut *(*dyd).label.arr.offset(i as isize) as *mut Labeldesc;
        if (*lb).name == (*gt).name {
            /* correct label? */
            if (*gt).nactvar as lua_int > (*lb).nactvar as lua_int
                && (0 != (*bl).upval as lua_int || (*dyd).label.n > (*bl).firstlabel)
            {
                luaK_patchclose((*ls).fs, (*gt).pc, (*lb).nactvar as lua_int);
            }
            /* close it */
            closegoto(ls, g, lb);
            return 1i32;
        } else {
            i += 1
        }
    }
    /* label not found; cannot close goto */
    return 0i32;
}
unsafe extern "C" fn closegoto(
    mut ls: *mut LexState,
    mut g: lua_int,
    mut label: *mut Labeldesc,
) -> () {
    let mut i: lua_int = 0;
    let mut fs: *mut FuncState = (*ls).fs;
    let mut gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let mut gt: *mut Labeldesc = &mut *(*gl).arr.offset(g as isize) as *mut Labeldesc;
    if ((*gt).nactvar as lua_int) < (*label).nactvar as lua_int {
        let mut vname: *mut TString = (*getlocvar(fs, (*gt).nactvar as lua_int)).varname;
        let mut msg: *const lua_char = luaO_pushfstring!(
            (*ls).L,
            s!(b"<goto %s> at line %d jumps into the scope of local \'%s\'\x00"),
            ((*gt).name as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
            (*gt).line,
            (vname as *mut lua_char)
                .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
        );
        semerror(ls, msg);
    } else {
        luaK_patchlist(fs, (*gt).pc, (*label).pc);
        /* remove goto from pending list */
        i = g;
        while i < (*gl).n - 1i32 {
            *(*gl).arr.offset(i as isize) = *(*gl).arr.offset((i + 1i32) as isize);
            i += 1
        }
        (*gl).n -= 1;
        return;
    };
}
unsafe extern "C" fn getlocvar(mut fs: *mut FuncState, mut i: lua_int) -> *mut LocVar {
    let mut idx: lua_int = (*(*(*(*fs).ls).dyd)
        .actvar
        .arr
        .offset(((*fs).firstlocal + i) as isize))
    .idx as lua_int;
    return &mut *(*(*fs).f).locvars.offset(idx as isize) as *mut LocVar;
}
unsafe extern "C" fn removevars(mut fs: *mut FuncState, mut tolevel: lua_int) -> () {
    (*(*(*fs).ls).dyd).actvar.n -= (*fs).nactvar as lua_int - tolevel;
    while (*fs).nactvar as lua_int > tolevel {
        (*fs).nactvar = (*fs).nactvar.wrapping_sub(1);
        (*getlocvar(fs, (*fs).nactvar as lua_int)).endpc = (*fs).pc
    }
}
/*
** create a label named 'break' to resolve break statements
*/
unsafe extern "C" fn breaklabel(mut ls: *mut LexState) -> () {
    let mut n: *mut TString = luaS_new((*ls).L, s!(b"break\x00"));
    let mut l: lua_int = newlabelentry(ls, &mut (*(*ls).dyd).label, n, 0i32, (*(*ls).fs).pc);
    findgotos(ls, &mut *(*(*ls).dyd).label.arr.offset(l as isize));
}
unsafe extern "C" fn newlabelentry(
    mut ls: *mut LexState,
    mut l: *mut Labellist,
    mut name: *mut TString,
    mut line: lua_int,
    mut pc: lua_int,
) -> lua_int {
    let mut n: lua_int = (*l).n;
    if n + 1i32 > (*l).size {
        (*l).arr = luaM_growaux_(
            (*ls).L,
            (*l).arr as *mut lua_void,
            &mut (*l).size,
            ::std::mem::size_of::<Labeldesc>() as lua_ulong,
            32767i32,
            s!(b"labels/gotos\x00"),
        ) as *mut Labeldesc
    }
    let ref mut fresh0 = (*(*l).arr.offset(n as isize)).name;
    *fresh0 = name;
    (*(*l).arr.offset(n as isize)).line = line;
    (*(*l).arr.offset(n as isize)).nactvar = (*(*ls).fs).nactvar;
    (*(*l).arr.offset(n as isize)).pc = pc;
    (*l).n = n + 1i32;
    return n;
}
/*
** check whether new label 'lb' matches any pending gotos in current
** block; solves forward jumps
*/
unsafe extern "C" fn findgotos(mut ls: *mut LexState, mut lb: *mut Labeldesc) -> () {
    let mut gl: *mut Labellist = &mut (*(*ls).dyd).gt;
    let mut i: lua_int = (*(*(*ls).fs).bl).firstgoto;
    while i < (*gl).n {
        if (*(*gl).arr.offset(i as isize)).name == (*lb).name {
            closegoto(ls, i, lb);
        } else {
            i += 1
        }
    }
}
unsafe extern "C" fn check(mut ls: *mut LexState, mut c: lua_int) -> () {
    if (*ls).t.token != c {
        error_expected(ls, c);
    } else {
        return;
    };
}
unsafe extern "C" fn error_expected(mut ls: *mut LexState, mut token: lua_int) -> ! {
    luaX_syntaxerror(
        ls,
        luaO_pushfstring!((*ls).L, s!(b"%s expected\x00"), luaX_token2str(ls, token),),
    );
}
unsafe extern "C" fn statlist(mut ls: *mut LexState) -> () {
    /* statlist -> { stat [';'] } */
    while 0 == block_follow(ls, 1i32) {
        if (*ls).t.token == TK_RETURN as lua_int {
            statement(ls);
            /* 'return' must be last statement */
            return;
        } else {
            statement(ls);
        }
    }
}
/*
** prototypes for recursive non-terminal functions
*/
unsafe extern "C" fn statement(mut ls: *mut LexState) -> () {
    /* may be needed for error messages */
    let mut line: lua_int = (*ls).linenumber;
    enterlevel(ls);
    match (*ls).t.token {
        59 => {
            /* stat -> ';' (empty statement) */
            /* skip ';' */
            luaX_next(ls);
        }
        267 => {
            /* stat -> ifstat */
            ifstat(ls, line);
        }
        278 => {
            /* stat -> whilestat */
            whilestat(ls, line);
        }
        259 => {
            /* stat -> DO block END */
            /* skip DO */
            luaX_next(ls);
            block(ls);
            check_match(ls, TK_END as lua_int, TK_DO as lua_int, line);
        }
        264 => {
            /* stat -> forstat */
            forstat(ls, line);
        }
        273 => {
            /* stat -> repeatstat */
            repeatstat(ls, line);
        }
        265 => {
            /* stat -> funcstat */
            funcstat(ls, line);
        }
        269 => {
            /* stat -> localstat */
            /* skip LOCAL */
            luaX_next(ls);
            /* local function? */
            if 0 != testnext(ls, TK_FUNCTION as lua_int) {
                localfunc(ls);
            } else {
                localstat(ls);
            }
        }
        288 => {
            /* stat -> label */
            /* skip double colon */
            luaX_next(ls);
            labelstat(ls, str_checkname(ls), line);
        }
        274 => {
            /* stat -> retstat */
            /* skip RETURN */
            luaX_next(ls);
            retstat(ls);
        }
        258 | 266 => {
            /* stat -> 'goto' NAME */
            gotostat(ls, luaK_jump((*ls).fs));
        }
        _ => {
            /* stat -> func | assignment */
            exprstat(ls);
        }
    }
    /* free registers */
    (*(*ls).fs).freereg = (*(*ls).fs).nactvar;
    (*(*ls).L).nCcalls = (*(*ls).L).nCcalls.wrapping_sub(1);
}
unsafe extern "C" fn exprstat(mut ls: *mut LexState) -> () {
    /* stat -> func | assignment */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut v: LHS_assign = LHS_assign {
        prev: 0 as *mut LHS_assign,
        v: expdesc {
            k: VVOID,
            u: expdesc_1 { ival: 0 },
            t: 0,
            f: 0,
        },
    };
    suffixedexp(ls, &mut v.v);
    if (*ls).t.token == '=' as i32 || (*ls).t.token == ',' as i32 {
        /* stat -> assignment ? */
        v.prev = 0 as *mut LHS_assign;
        assignment(ls, &mut v, 1i32);
    } else if !(v.v.k as lua_uint == VCALL as lua_int as lua_uint) {
        luaX_syntaxerror(ls, s!(b"syntax error\x00"));
    } else {
        *(*(*fs).f).code.offset(v.v.u.info as isize) = *(*(*fs).f).code.offset(v.v.u.info as isize)
            & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32)
            | (1i32 as Instruction) << 0i32 + 6i32 + 8i32
                & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32
    };
}
unsafe extern "C" fn assignment(
    mut ls: *mut LexState,
    mut lh: *mut LHS_assign,
    mut nvars: lua_int,
) -> () {
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    if !(VLOCAL as lua_int as lua_uint <= (*lh).v.k as lua_uint
        && (*lh).v.k as lua_uint <= VINDEXED as lua_int as lua_uint)
    {
        luaX_syntaxerror(ls, s!(b"syntax error\x00"));
    } else {
        if 0 != testnext(ls, ',' as i32) {
            /* assignment -> ',' suffixedexp assignment */
            let mut nv: LHS_assign = LHS_assign {
                prev: 0 as *mut LHS_assign,
                v: expdesc {
                    k: VVOID,
                    u: expdesc_1 { ival: 0 },
                    t: 0,
                    f: 0,
                },
            };
            nv.prev = lh;
            suffixedexp(ls, &mut nv.v);
            if nv.v.k as lua_uint != VINDEXED as lua_int as lua_uint {
                check_conflict(ls, lh, &mut nv.v);
            }
            checklimit(
                (*ls).fs,
                nvars + (*(*ls).L).nCcalls as lua_int,
                LUAI_MAXCCALLS,
                s!(b"C levels\x00"),
            );
            assignment(ls, &mut nv, nvars + 1i32);
        } else {
            /* assignment -> '=' explist */
            let mut nexps: lua_int = 0;
            checknext(ls, '=' as i32);
            nexps = explist(ls, &mut e);
            if nexps != nvars {
                adjust_assign(ls, nvars, nexps, &mut e);
            } else {
                /* close last expression */
                luaK_setoneret((*ls).fs, &mut e);
                luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
                /* avoid default */
                return;
            }
        }
        /* default assignment */
        init_exp(&mut e, VNONRELOC, (*(*ls).fs).freereg as lua_int - 1i32);
        luaK_storevar((*ls).fs, &mut (*lh).v, &mut e);
        return;
    };
}
unsafe extern "C" fn init_exp(mut e: *mut expdesc, mut k: expkind, mut i: lua_int) -> () {
    (*e).t = -1i32;
    (*e).f = (*e).t;
    (*e).k = k;
    (*e).u.info = i;
}
unsafe extern "C" fn adjust_assign(
    mut ls: *mut LexState,
    mut nvars: lua_int,
    mut nexps: lua_int,
    mut e: *mut expdesc,
) -> () {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut extra: lua_int = nvars - nexps;
    if (*e).k as lua_uint == VCALL as lua_int as lua_uint
        || (*e).k as lua_uint == VVARARG as lua_int as lua_uint
    {
        /* includes call itself */
        extra += 1;
        if extra < 0i32 {
            extra = 0i32
        }
        /* last exp. provides the difference */
        luaK_setreturns(fs, e, extra);
        if extra > 1i32 {
            luaK_reserveregs(fs, extra - 1i32);
        }
    } else {
        if (*e).k as lua_uint != VVOID as lua_int as lua_uint {
            /* close last expression */
            luaK_exp2nextreg(fs, e);
        }
        if extra > 0i32 {
            let mut reg: lua_int = (*fs).freereg as lua_int;
            luaK_reserveregs(fs, extra);
            luaK_nil(fs, reg, extra);
        }
    }
    if nexps > nvars {
        /* remove extra values */
        (*(*ls).fs).freereg = ((*(*ls).fs).freereg as lua_int - (nexps - nvars)) as lu_byte
    };
}
unsafe extern "C" fn explist(mut ls: *mut LexState, mut v: *mut expdesc) -> lua_int {
    /* explist -> expr { ',' expr } */
    /* at least one expression */
    let mut n: lua_int = 1i32;
    expr(ls, v);
    while 0 != testnext(ls, ',' as i32) {
        luaK_exp2nextreg((*ls).fs, v);
        expr(ls, v);
        n += 1
    }
    return n;
}
unsafe extern "C" fn expr(mut ls: *mut LexState, mut v: *mut expdesc) -> () {
    subexpr(ls, v, 0i32);
}
/* '+' '-' */
/* '*' '%' */
/* '^' (right associative) */
/* '/' '//' */
/* '&' '|' '~' */
/* '<<' '>>' */
/* '..' (right associative) */
/* ==, <, <= */
/* ~=, >, >= */
/* and, or */
/* priority for unary operators */
/*
** subexpr -> (simpleexp | unop subexpr) { binop subexpr }
** where 'binop' is any binary operator with a priority higher than 'limit'
*/
unsafe extern "C" fn subexpr(
    mut ls: *mut LexState,
    mut v: *mut expdesc,
    mut limit: lua_int,
) -> BinOpr {
    let mut line: lua_int = 0;
    let mut v2: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut nextop: BinOpr = OPR_ADD;
    let mut line_0: lua_int = 0;
    let mut op: BinOpr = OPR_ADD;
    let mut uop: UnOpr = OPR_MINUS;
    enterlevel(ls);
    uop = getunopr((*ls).t.token);
    if uop as lua_uint != OPR_NOUNOPR as lua_int as lua_uint {
        line = (*ls).linenumber;
        luaX_next(ls);
        subexpr(ls, v, 12i32);
        luaK_prefix((*ls).fs, uop, v, line);
    } else {
        simpleexp(ls, v);
    }
    /* expand while operators have priorities higher than 'limit' */
    op = getbinopr((*ls).t.token);
    while op as lua_uint != OPR_NOBINOPR as lua_int as lua_uint
        && priority[op as usize].left as lua_int > limit
    {
        v2 = expdesc {
            k: VVOID,
            u: expdesc_1 { ival: 0 },
            t: 0,
            f: 0,
        };
        nextop = OPR_ADD;
        line_0 = (*ls).linenumber;
        luaX_next(ls);
        luaK_infix((*ls).fs, op, v);
        /* read sub-expression with higher priority */
        nextop = subexpr(ls, &mut v2, priority[op as usize].right as lua_int);
        luaK_posfix((*ls).fs, op, v, &mut v2, line_0);
        op = nextop
    }
    (*(*ls).L).nCcalls = (*(*ls).L).nCcalls.wrapping_sub(1);
    /* return first untreated operator */
    return op;
}
/* ORDER OPR */
static mut priority: [unnamed_9; 21] = [
    unnamed_9 {
        left: 10i32 as lu_byte,
        right: 10i32 as lu_byte,
    },
    unnamed_9 {
        left: 10i32 as lu_byte,
        right: 10i32 as lu_byte,
    },
    unnamed_9 {
        left: 11i32 as lu_byte,
        right: 11i32 as lu_byte,
    },
    unnamed_9 {
        left: 11i32 as lu_byte,
        right: 11i32 as lu_byte,
    },
    unnamed_9 {
        left: 14i32 as lu_byte,
        right: 13i32 as lu_byte,
    },
    unnamed_9 {
        left: 11i32 as lu_byte,
        right: 11i32 as lu_byte,
    },
    unnamed_9 {
        left: 11i32 as lu_byte,
        right: 11i32 as lu_byte,
    },
    unnamed_9 {
        left: 6i32 as lu_byte,
        right: 6i32 as lu_byte,
    },
    unnamed_9 {
        left: 4i32 as lu_byte,
        right: 4i32 as lu_byte,
    },
    unnamed_9 {
        left: 5i32 as lu_byte,
        right: 5i32 as lu_byte,
    },
    unnamed_9 {
        left: 7i32 as lu_byte,
        right: 7i32 as lu_byte,
    },
    unnamed_9 {
        left: 7i32 as lu_byte,
        right: 7i32 as lu_byte,
    },
    unnamed_9 {
        left: 9i32 as lu_byte,
        right: 8i32 as lu_byte,
    },
    unnamed_9 {
        left: 3i32 as lu_byte,
        right: 3i32 as lu_byte,
    },
    unnamed_9 {
        left: 3i32 as lu_byte,
        right: 3i32 as lu_byte,
    },
    unnamed_9 {
        left: 3i32 as lu_byte,
        right: 3i32 as lu_byte,
    },
    unnamed_9 {
        left: 3i32 as lu_byte,
        right: 3i32 as lu_byte,
    },
    unnamed_9 {
        left: 3i32 as lu_byte,
        right: 3i32 as lu_byte,
    },
    unnamed_9 {
        left: 3i32 as lu_byte,
        right: 3i32 as lu_byte,
    },
    unnamed_9 {
        left: 2i32 as lu_byte,
        right: 2i32 as lu_byte,
    },
    unnamed_9 {
        left: 1i32 as lu_byte,
        right: 1i32 as lu_byte,
    },
];
unsafe extern "C" fn getbinopr(mut op: lua_int) -> BinOpr {
    match op {
        43 => return OPR_ADD,
        45 => return OPR_SUB,
        42 => return OPR_MUL,
        37 => return OPR_MOD,
        94 => return OPR_POW,
        47 => return OPR_DIV,
        279 => return OPR_IDIV,
        38 => return OPR_BAND,
        124 => return OPR_BOR,
        126 => return OPR_BXOR,
        286 => return OPR_SHL,
        287 => return OPR_SHR,
        280 => return OPR_CONCAT,
        285 => return OPR_NE,
        282 => return OPR_EQ,
        60 => return OPR_LT,
        284 => return OPR_LE,
        62 => return OPR_GT,
        283 => return OPR_GE,
        257 => return OPR_AND,
        272 => return OPR_OR,
        _ => return OPR_NOBINOPR,
    };
}
unsafe extern "C" fn simpleexp(mut ls: *mut LexState, mut v: *mut expdesc) -> () {
    /* simpleexp -> FLT | INT | STRING | NIL | TRUE | FALSE | ... |
    constructor | FUNCTION body | suffixedexp */
    match (*ls).t.token {
        290 => {
            init_exp(v, VKFLT, 0i32);
            (*v).u.nval = (*ls).t.seminfo.r
        }
        291 => {
            init_exp(v, VKINT, 0i32);
            (*v).u.ival = (*ls).t.seminfo.i
        }
        293 => {
            codestring(ls, v, (*ls).t.seminfo.ts);
        }
        270 => {
            init_exp(v, VNIL, 0i32);
        }
        276 => {
            init_exp(v, VTRUE, 0i32);
        }
        263 => {
            init_exp(v, VFALSE, 0i32);
        }
        281 => {
            /* vararg */
            let mut fs: *mut FuncState = (*ls).fs;
            if 0 == (*(*fs).f).is_vararg {
                luaX_syntaxerror(ls, s!(b"cannot use \'...\' outside a vararg function\x00"));
            } else {
                init_exp(v, VVARARG, luaK_codeABC(fs, OP_VARARG, 0i32, 1i32, 0i32));
            }
        }
        123 => {
            /* constructor */
            constructor(ls, v);
            return;
        }
        265 => {
            luaX_next(ls);
            body(ls, v, 0i32, (*ls).linenumber);
            return;
        }
        _ => {
            suffixedexp(ls, v);
            return;
        }
    }
    luaX_next(ls);
}
unsafe extern "C" fn suffixedexp(mut ls: *mut LexState, mut v: *mut expdesc) -> () {
    /* suffixedexp ->
    primaryexp { '.' NAME | '[' exp ']' | ':' NAME funcargs | funcargs } */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut line: lua_int = (*ls).linenumber;
    primaryexp(ls, v);
    loop {
        match (*ls).t.token {
            46 => {
                /* fieldsel */
                fieldsel(ls, v);
            }
            91 => {
                /* '[' exp1 ']' */
                let mut key: expdesc = expdesc {
                    k: VVOID,
                    u: expdesc_1 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaK_exp2anyregup(fs, v);
                yindex(ls, &mut key);
                luaK_indexed(fs, v, &mut key);
            }
            58 => {
                /* ':' NAME funcargs */
                let mut key_0: expdesc = expdesc {
                    k: VVOID,
                    u: expdesc_1 { ival: 0 },
                    t: 0,
                    f: 0,
                };
                luaX_next(ls);
                checkname(ls, &mut key_0);
                luaK_self(fs, v, &mut key_0);
                funcargs(ls, v, line);
            }
            40 | 293 | 123 => {
                /* funcargs */
                luaK_exp2nextreg(fs, v);
                funcargs(ls, v, line);
            }
            _ => return,
        }
    }
}
unsafe extern "C" fn funcargs(mut ls: *mut LexState, mut f: *mut expdesc, mut line: lua_int) -> () {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut args: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut base: lua_int = 0;
    let mut nparams: lua_int = 0;
    match (*ls).t.token {
        40 => {
            /* funcargs -> '(' [ explist ] ')' */
            luaX_next(ls);
            /* arg list is empty? */
            if (*ls).t.token == ')' as i32 {
                args.k = VVOID
            } else {
                explist(ls, &mut args);
                luaK_setreturns(fs, &mut args, -1i32);
            }
            check_match(ls, ')' as i32, '(' as i32, line);
        }
        123 => {
            /* funcargs -> constructor */
            constructor(ls, &mut args);
        }
        293 => {
            /* funcargs -> STRING */
            codestring(ls, &mut args, (*ls).t.seminfo.ts);
            /* must use 'seminfo' before 'next' */
            luaX_next(ls);
        }
        _ => {
            luaX_syntaxerror(ls, s!(b"function arguments expected\x00"));
        }
    }
    /* base register for call */
    base = (*f).u.info;
    if args.k as lua_uint == VCALL as lua_int as lua_uint
        || args.k as lua_uint == VVARARG as lua_int as lua_uint
    {
        /* open call */
        nparams = -1i32
    } else {
        if args.k as lua_uint != VVOID as lua_int as lua_uint {
            /* close last argument */
            luaK_exp2nextreg(fs, &mut args);
        }
        nparams = (*fs).freereg as lua_int - (base + 1i32)
    }
    init_exp(
        f,
        VCALL,
        luaK_codeABC(fs, OP_CALL, base, nparams + 1i32, 2i32),
    );
    luaK_fixline(fs, line);
    /* call remove function and arguments and leaves
    (unless changed) one result */
    (*fs).freereg = (base + 1i32) as lu_byte;
}
unsafe extern "C" fn codestring(
    mut ls: *mut LexState,
    mut e: *mut expdesc,
    mut s: *mut TString,
) -> () {
    init_exp(e, VK, luaK_stringK((*ls).fs, s));
}
unsafe extern "C" fn constructor(mut ls: *mut LexState, mut t: *mut expdesc) -> () {
    /* constructor -> '{' [ field { sep field } [sep] ] '}'
    sep -> ',' | ';' */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut line: lua_int = (*ls).linenumber;
    let mut pc: lua_int = luaK_codeABC(fs, OP_NEWTABLE, 0i32, 0i32, 0i32);
    let mut cc: ConsControl = ConsControl {
        v: expdesc {
            k: VVOID,
            u: expdesc_1 { ival: 0 },
            t: 0,
            f: 0,
        },
        t: 0 as *mut expdesc,
        nh: 0,
        na: 0,
        tostore: 0,
    };
    cc.tostore = 0i32;
    cc.nh = cc.tostore;
    cc.na = cc.nh;
    cc.t = t;
    init_exp(t, VRELOCABLE, pc);
    /* no value (yet) */
    init_exp(&mut cc.v, VVOID, 0i32);
    /* fix it at stack top */
    luaK_exp2nextreg((*ls).fs, t);
    checknext(ls, '{' as i32);
    while !((*ls).t.token == '}' as i32) {
        closelistfield(fs, &mut cc);
        field(ls, &mut cc);
        if !(0 != testnext(ls, ',' as i32) || 0 != testnext(ls, ';' as i32)) {
            break;
        }
    }
    check_match(ls, '}' as i32, '{' as i32, line);
    lastlistfield(fs, &mut cc);
    /* set initial array size */
    *(*(*fs).f).code.offset(pc as isize) = *(*(*fs).f).code.offset(pc as isize)
        & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32)
        | (luaO_int2fb(cc.na as lua_uint) as Instruction) << 0i32 + 6i32 + 8i32 + 9i32
            & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32 + 9i32;
    /* set initial table size */
    *(*(*fs).f).code.offset(pc as isize) = *(*(*fs).f).code.offset(pc as isize)
        & !(!((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32)
        | (luaO_int2fb(cc.nh as lua_uint) as Instruction) << 0i32 + 6i32 + 8i32
            & !((!(0i32 as Instruction)) << 9i32) << 0i32 + 6i32 + 8i32;
}
unsafe extern "C" fn lastlistfield(mut fs: *mut FuncState, mut cc: *mut ConsControl) -> () {
    if (*cc).tostore == 0i32 {
        return;
    } else {
        if (*cc).v.k as lua_uint == VCALL as lua_int as lua_uint
            || (*cc).v.k as lua_uint == VVARARG as lua_int as lua_uint
        {
            luaK_setreturns(fs, &mut (*cc).v, -1i32);
            luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, -1i32);
            /* do not count last expression (unknown number of elements) */
            (*cc).na -= 1
        } else {
            if (*cc).v.k as lua_uint != VVOID as lua_int as lua_uint {
                luaK_exp2nextreg(fs, &mut (*cc).v);
            }
            luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
        }
        return;
    };
}
unsafe extern "C" fn check_match(
    mut ls: *mut LexState,
    mut what: lua_int,
    mut who: lua_int,
    mut where_0: lua_int,
) -> () {
    if 0 == testnext(ls, what) {
        if where_0 == (*ls).linenumber {
            error_expected(ls, what);
        } else {
            luaX_syntaxerror(
                ls,
                luaO_pushfstring!(
                    (*ls).L,
                    s!(b"%s expected (to close %s at line %d)\x00"),
                    luaX_token2str(ls, what),
                    luaX_token2str(ls, who),
                    where_0,
                ),
            );
        }
    } else {
        return;
    };
}
unsafe extern "C" fn testnext(mut ls: *mut LexState, mut c: lua_int) -> lua_int {
    if (*ls).t.token == c {
        luaX_next(ls);
        return 1i32;
    } else {
        return 0i32;
    };
}
unsafe extern "C" fn field(mut ls: *mut LexState, mut cc: *mut ConsControl) -> () {
    /* field -> listfield | recfield */
    match (*ls).t.token {
        292 => {
            /* may be 'listfield' or 'recfield' */
            /* expression? */
            if luaX_lookahead(ls) != '=' as i32 {
                listfield(ls, cc);
            } else {
                recfield(ls, cc);
            }
        }
        91 => {
            recfield(ls, cc);
        }
        _ => {
            listfield(ls, cc);
        }
    };
}
unsafe extern "C" fn listfield(mut ls: *mut LexState, mut cc: *mut ConsControl) -> () {
    /* listfield -> exp */
    expr(ls, &mut (*cc).v);
    checklimit(
        (*ls).fs,
        (*cc).na,
        2147483647i32,
        s!(b"items in a constructor\x00"),
    );
    (*cc).na += 1;
    (*cc).tostore += 1;
}
unsafe extern "C" fn checklimit(
    mut fs: *mut FuncState,
    mut v: lua_int,
    mut l: lua_int,
    mut what: *const lua_char,
) -> () {
    if v > l {
        errorlimit(fs, l, what);
    } else {
        return;
    };
}
unsafe extern "C" fn errorlimit(
    mut fs: *mut FuncState,
    mut limit: lua_int,
    mut what: *const lua_char,
) -> ! {
    let mut L: *mut lua_State = (*(*fs).ls).L;
    let mut msg: *const lua_char = 0 as *const lua_char;
    let mut line: lua_int = (*(*fs).f).linedefined;
    let mut where_0: *const lua_char = if line == 0i32 {
        s!(b"main function\x00")
    } else {
        luaO_pushfstring!(L, s!(b"function at line %d\x00"), line,)
    };
    msg = luaO_pushfstring!(
        L,
        s!(b"too many %s (limit is %d) in %s\x00"),
        what,
        limit,
        where_0,
    );
    luaX_syntaxerror((*fs).ls, msg);
}
unsafe extern "C" fn recfield(mut ls: *mut LexState, mut cc: *mut ConsControl) -> () {
    /* recfield -> (NAME | '['exp1']') = exp1 */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut reg: lua_int = (*(*ls).fs).freereg as lua_int;
    let mut key: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut val: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut rkkey: lua_int = 0;
    if (*ls).t.token == TK_NAME as lua_int {
        checklimit(
            fs,
            (*cc).nh,
            2147483647i32,
            s!(b"items in a constructor\x00"),
        );
        checkname(ls, &mut key);
    } else {
        /* ls->t.token == '[' */
        yindex(ls, &mut key);
    }
    (*cc).nh += 1;
    checknext(ls, '=' as i32);
    rkkey = luaK_exp2RK(fs, &mut key);
    expr(ls, &mut val);
    luaK_codeABC(
        fs,
        OP_SETTABLE,
        (*(*cc).t).u.info,
        rkkey,
        luaK_exp2RK(fs, &mut val),
    );
    /* free registers */
    (*fs).freereg = reg as lu_byte;
}
unsafe extern "C" fn checknext(mut ls: *mut LexState, mut c: lua_int) -> () {
    check(ls, c);
    luaX_next(ls);
}
unsafe extern "C" fn yindex(mut ls: *mut LexState, mut v: *mut expdesc) -> () {
    /* index -> '[' expr ']' */
    /* skip the '[' */
    luaX_next(ls);
    expr(ls, v);
    luaK_exp2val((*ls).fs, v);
    checknext(ls, ']' as i32);
}
unsafe extern "C" fn checkname(mut ls: *mut LexState, mut e: *mut expdesc) -> () {
    codestring(ls, e, str_checkname(ls));
}
unsafe extern "C" fn str_checkname(mut ls: *mut LexState) -> *mut TString {
    let mut ts: *mut TString = 0 as *mut TString;
    check(ls, TK_NAME as lua_int);
    ts = (*ls).t.seminfo.ts;
    luaX_next(ls);
    return ts;
}
unsafe extern "C" fn closelistfield(mut fs: *mut FuncState, mut cc: *mut ConsControl) -> () {
    if (*cc).v.k as lua_uint == VVOID as lua_int as lua_uint {
        /* there is no list item */
        return;
    } else {
        luaK_exp2nextreg(fs, &mut (*cc).v);
        (*cc).v.k = VVOID;
        if (*cc).tostore == 50i32 {
            /* flush */
            luaK_setlist(fs, (*(*cc).t).u.info, (*cc).na, (*cc).tostore);
            /* no more items pending */
            (*cc).tostore = 0i32
        }
        return;
    };
}
unsafe extern "C" fn fieldsel(mut ls: *mut LexState, mut v: *mut expdesc) -> () {
    /* fieldsel -> ['.' | ':'] NAME */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut key: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    luaK_exp2anyregup(fs, v);
    /* skip the dot or colon */
    luaX_next(ls);
    checkname(ls, &mut key);
    luaK_indexed(fs, v, &mut key);
}
/*
** {======================================================================
** Expression parsing
** =======================================================================
*/
unsafe extern "C" fn primaryexp(mut ls: *mut LexState, mut v: *mut expdesc) -> () {
    /* primaryexp -> NAME | '(' expr ')' */
    match (*ls).t.token {
        40 => {
            let mut line: lua_int = (*ls).linenumber;
            luaX_next(ls);
            expr(ls, v);
            check_match(ls, ')' as i32, '(' as i32, line);
            luaK_dischargevars((*ls).fs, v);
            return;
        }
        292 => {
            singlevar(ls, v);
            return;
        }
        _ => {
            luaX_syntaxerror(ls, s!(b"unexpected symbol\x00"));
        }
    };
}
unsafe extern "C" fn singlevar(mut ls: *mut LexState, mut var: *mut expdesc) -> () {
    let mut varname: *mut TString = str_checkname(ls);
    let mut fs: *mut FuncState = (*ls).fs;
    singlevaraux(fs, varname, var, 1i32);
    if (*var).k as lua_uint == VVOID as lua_int as lua_uint {
        /* global name? */
        let mut key: expdesc = expdesc {
            k: VVOID,
            u: expdesc_1 { ival: 0 },
            t: 0,
            f: 0,
        };
        /* get environment variable */
        singlevaraux(fs, (*ls).envn, var, 1i32);
        /* this one must exist */
        /* key is variable name */
        codestring(ls, &mut key, varname);
        /* env[varname] */
        luaK_indexed(fs, var, &mut key);
    };
}
/*
  Find variable with given name 'n'. If it is an upvalue, add this
  upvalue into all intermediate functions.
*/
unsafe extern "C" fn singlevaraux(
    mut fs: *mut FuncState,
    mut n: *mut TString,
    mut var: *mut expdesc,
    mut base: lua_int,
) -> () {
    /* no more levels? */
    if fs.is_null() {
        /* default is global */
        init_exp(var, VVOID, 0i32);
    } else {
        /* look up locals at current level */
        let mut v: lua_int = searchvar(fs, n);
        if v >= 0i32 {
            /* found? */
            /* variable is local */
            init_exp(var, VLOCAL, v);
            if 0 == base {
                /* local will be used as an upval */
                markupval(fs, v);
            }
        } else {
            /* not found as local at current level; try upvalues */
            /* try existing upvalues */
            let mut idx: lua_int = searchupvalue(fs, n);
            if idx < 0i32 {
                /* not found? */
                /* try upper levels */
                singlevaraux((*fs).prev, n, var, 0i32);
                /* not found? */
                if (*var).k as lua_uint == VVOID as lua_int as lua_uint {
                    /* it is a global */
                    return;
                } else {
                    idx = newupvalue(fs, n, var)
                }
            }
            /* new or old upvalue */
            init_exp(var, VUPVAL, idx);
        }
    };
}
unsafe extern "C" fn searchupvalue(mut fs: *mut FuncState, mut name: *mut TString) -> lua_int {
    let mut i: lua_int = 0;
    let mut up: *mut Upvaldesc = (*(*fs).f).upvalues;
    i = 0i32;
    while i < (*fs).nups as lua_int {
        if (*up.offset(i as isize)).name == name {
            return i;
        } else {
            i += 1
        }
    }
    /* not found */
    return -1i32;
}
unsafe extern "C" fn newupvalue(
    mut fs: *mut FuncState,
    mut name: *mut TString,
    mut v: *mut expdesc,
) -> lua_int {
    let mut f: *mut Proto = (*fs).f;
    let mut oldsize: lua_int = (*f).sizeupvalues;
    checklimit(
        fs,
        (*fs).nups as lua_int + 1i32,
        255i32,
        s!(b"upvalues\x00"),
    );
    if (*fs).nups as lua_int + 1i32 > (*f).sizeupvalues {
        (*f).upvalues = luaM_growaux_(
            (*(*fs).ls).L,
            (*f).upvalues as *mut lua_void,
            &mut (*f).sizeupvalues,
            ::std::mem::size_of::<Upvaldesc>() as lua_ulong,
            255i32,
            s!(b"upvalues\x00"),
        ) as *mut Upvaldesc
    }
    while oldsize < (*f).sizeupvalues {
        let fresh1 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh2 = (*(*f).upvalues.offset(fresh1 as isize)).name;
        *fresh2 = 0 as *mut TString
    }
    (*(*f).upvalues.offset((*fs).nups as isize)).instack =
        ((*v).k as lua_uint == VLOCAL as lua_int as lua_uint) as lua_int as lu_byte;
    (*(*f).upvalues.offset((*fs).nups as isize)).idx = (*v).u.info as lu_byte;
    let ref mut fresh3 = (*(*f).upvalues.offset((*fs).nups as isize)).name;
    *fresh3 = name;
    if 0 != (*f).marked as lua_int & 1i32 << 2i32
        && 0 != (*name).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrier_(
            (*(*fs).ls).L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*(name as *mut GCUnion)).gc,
        );
    } else {
    };
    let fresh4 = (*fs).nups;
    (*fs).nups = (*fs).nups.wrapping_add(1);
    return fresh4 as lua_int;
}
unsafe extern "C" fn searchvar(mut fs: *mut FuncState, mut n: *mut TString) -> lua_int {
    let mut i: lua_int = 0;
    i = (*fs).nactvar as lua_int - 1i32;
    while i >= 0i32 {
        if n == (*getlocvar(fs, i)).varname {
            return i;
        } else {
            i -= 1
        }
    }
    /* not found */
    return -1i32;
}
/*
  Mark block where variable at given level was defined
  (to emit close instructions later).
*/
unsafe extern "C" fn markupval(mut fs: *mut FuncState, mut level: lua_int) -> () {
    let mut bl: *mut BlockCnt = (*fs).bl;
    while (*bl).nactvar as lua_int > level {
        bl = (*bl).previous
    }
    (*bl).upval = 1i32 as lu_byte;
}
unsafe extern "C" fn body(
    mut ls: *mut LexState,
    mut e: *mut expdesc,
    mut ismethod: lua_int,
    mut line: lua_int,
) -> () {
    /* body ->  '(' parlist ')' block END */
    let mut new_fs: FuncState = FuncState {
        f: 0 as *mut Proto,
        prev: 0 as *mut FuncState,
        ls: 0 as *mut LexState,
        bl: 0 as *mut BlockCnt,
        pc: 0,
        lasttarget: 0,
        jpc: 0,
        nk: 0,
        np: 0,
        firstlocal: 0,
        nlocvars: 0,
        nactvar: 0,
        nups: 0,
        freereg: 0,
    };
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    new_fs.f = addprototype(ls);
    (*new_fs.f).linedefined = line;
    open_func(ls, &mut new_fs, &mut bl);
    checknext(ls, '(' as i32);
    if 0 != ismethod {
        /* create 'self' parameter */
        new_localvarliteral_(
            ls,
            s!(b"self\x00"),
            (::std::mem::size_of::<[lua_char; 5]>() as lua_ulong)
                .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
                .wrapping_sub(1i32 as lua_ulong),
        );
        adjustlocalvars(ls, 1i32);
    }
    parlist(ls);
    checknext(ls, ')' as i32);
    statlist(ls);
    (*new_fs.f).lastlinedefined = (*ls).linenumber;
    check_match(ls, TK_END as lua_int, TK_FUNCTION as lua_int, line);
    codeclosure(ls, e);
    close_func(ls);
}
/*
** codes instruction to create new closure in parent function.
** The OP_CLOSURE instruction must use the last available register,
** so that, if it invokes the GC, the GC knows which registers
** are in use at that time.
*/
unsafe extern "C" fn codeclosure(mut ls: *mut LexState, mut v: *mut expdesc) -> () {
    let mut fs: *mut FuncState = (*(*ls).fs).prev;
    init_exp(
        v,
        VRELOCABLE,
        luaK_codeABx(fs, OP_CLOSURE, 0i32, ((*fs).np - 1i32) as lua_uint),
    );
    /* fix it at the last register */
    luaK_exp2nextreg(fs, v);
}
/* }====================================================================== */
unsafe extern "C" fn parlist(mut ls: *mut LexState) -> () {
    /* parlist -> [ param { ',' param } ] */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut f: *mut Proto = (*fs).f;
    let mut nparams: lua_int = 0i32;
    (*f).is_vararg = 0i32 as lu_byte;
    if (*ls).t.token != ')' as i32 {
        /* is 'parlist' not empty? */
        loop {
            match (*ls).t.token {
                292 => {
                    /* param -> NAME */
                    new_localvar(ls, str_checkname(ls));
                    nparams += 1
                }
                281 => {
                    /* param -> '...' */
                    luaX_next(ls);
                    /* declared vararg */
                    (*f).is_vararg = 1i32 as lu_byte
                }
                _ => {
                    luaX_syntaxerror(ls, s!(b"<name> or \'...\' expected\x00"));
                }
            }
            if !(0 == (*f).is_vararg && 0 != testnext(ls, ',' as i32)) {
                break;
            }
        }
    }
    adjustlocalvars(ls, nparams);
    (*f).numparams = (*fs).nactvar;
    /* reserve register for parameters */
    luaK_reserveregs(fs, (*fs).nactvar as lua_int);
}
unsafe extern "C" fn adjustlocalvars(mut ls: *mut LexState, mut nvars: lua_int) -> () {
    let mut fs: *mut FuncState = (*ls).fs;
    (*fs).nactvar = ((*fs).nactvar as lua_int + nvars) as lu_byte;
    while 0 != nvars {
        (*getlocvar(fs, (*fs).nactvar as lua_int - nvars)).startpc = (*fs).pc;
        nvars -= 1
    }
}
unsafe extern "C" fn new_localvar(mut ls: *mut LexState, mut name: *mut TString) -> () {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut dyd: *mut Dyndata = (*ls).dyd;
    let mut reg: lua_int = registerlocalvar(ls, name);
    checklimit(
        fs,
        (*dyd).actvar.n + 1i32 - (*fs).firstlocal,
        200i32,
        s!(b"local variables\x00"),
    );
    if (*dyd).actvar.n + 1i32 + 1i32 > (*dyd).actvar.size {
        (*dyd).actvar.arr = luaM_growaux_(
            (*ls).L,
            (*dyd).actvar.arr as *mut lua_void,
            &mut (*dyd).actvar.size,
            ::std::mem::size_of::<Vardesc>() as lua_ulong,
            2147483647i32,
            s!(b"local variables\x00"),
        ) as *mut Vardesc
    }
    let fresh5 = (*dyd).actvar.n;
    (*dyd).actvar.n = (*dyd).actvar.n + 1;
    (*(*dyd).actvar.arr.offset(fresh5 as isize)).idx = reg as lua_short;
}
unsafe extern "C" fn registerlocalvar(mut ls: *mut LexState, mut varname: *mut TString) -> lua_int {
    let mut fs: *mut FuncState = (*ls).fs;
    let mut f: *mut Proto = (*fs).f;
    let mut oldsize: lua_int = (*f).sizelocvars;
    if (*fs).nlocvars as lua_int + 1i32 > (*f).sizelocvars {
        (*f).locvars = luaM_growaux_(
            (*ls).L,
            (*f).locvars as *mut lua_void,
            &mut (*f).sizelocvars,
            ::std::mem::size_of::<LocVar>() as lua_ulong,
            32767i32,
            s!(b"local variables\x00"),
        ) as *mut LocVar
    }
    while oldsize < (*f).sizelocvars {
        let fresh6 = oldsize;
        oldsize = oldsize + 1;
        let ref mut fresh7 = (*(*f).locvars.offset(fresh6 as isize)).varname;
        *fresh7 = 0 as *mut TString
    }
    let ref mut fresh8 = (*(*f).locvars.offset((*fs).nlocvars as isize)).varname;
    *fresh8 = varname;
    if 0 != (*f).marked as lua_int & 1i32 << 2i32
        && 0 != (*varname).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrier_(
            (*ls).L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*(varname as *mut GCUnion)).gc,
        );
    } else {
    };
    let fresh9 = (*fs).nlocvars;
    (*fs).nlocvars = (*fs).nlocvars + 1;
    return fresh9 as lua_int;
}
unsafe extern "C" fn new_localvarliteral_(
    mut ls: *mut LexState,
    mut name: *const lua_char,
    mut sz: size_t,
) -> () {
    new_localvar(ls, luaX_newstring(ls, name, sz));
}
unsafe extern "C" fn open_func(
    mut ls: *mut LexState,
    mut fs: *mut FuncState,
    mut bl: *mut BlockCnt,
) -> () {
    let mut f: *mut Proto = 0 as *mut Proto;
    /* linked list of funcstates */
    (*fs).prev = (*ls).fs;
    (*fs).ls = ls;
    (*ls).fs = fs;
    (*fs).pc = 0i32;
    (*fs).lasttarget = 0i32;
    (*fs).jpc = -1i32;
    (*fs).freereg = 0i32 as lu_byte;
    (*fs).nk = 0i32;
    (*fs).np = 0i32;
    (*fs).nups = 0i32 as lu_byte;
    (*fs).nlocvars = 0i32 as lua_short;
    (*fs).nactvar = 0i32 as lu_byte;
    (*fs).firstlocal = (*(*ls).dyd).actvar.n;
    (*fs).bl = 0 as *mut BlockCnt;
    f = (*fs).f;
    (*f).source = (*ls).source;
    /* registers 0/1 are always valid */
    (*f).maxstacksize = 2i32 as lu_byte;
    enterblock(fs, bl, 0i32 as lu_byte);
}
unsafe extern "C" fn enterblock(
    mut fs: *mut FuncState,
    mut bl: *mut BlockCnt,
    mut isloop: lu_byte,
) -> () {
    (*bl).isloop = isloop;
    (*bl).nactvar = (*fs).nactvar;
    (*bl).firstlabel = (*(*(*fs).ls).dyd).label.n;
    (*bl).firstgoto = (*(*(*fs).ls).dyd).gt.n;
    (*bl).upval = 0i32 as lu_byte;
    (*bl).previous = (*fs).bl;
    (*fs).bl = bl;
}
/*
** adds a new prototype into list of prototypes
*/
unsafe extern "C" fn addprototype(mut ls: *mut LexState) -> *mut Proto {
    let mut clp: *mut Proto = 0 as *mut Proto;
    let mut L: *mut lua_State = (*ls).L;
    let mut fs: *mut FuncState = (*ls).fs;
    /* prototype of current function */
    let mut f: *mut Proto = (*fs).f;
    if (*fs).np >= (*f).sizep {
        let mut oldsize: lua_int = (*f).sizep;
        if (*fs).np + 1i32 > (*f).sizep {
            (*f).p = luaM_growaux_(
                L,
                (*f).p as *mut lua_void,
                &mut (*f).sizep,
                ::std::mem::size_of::<*mut Proto>() as lua_ulong,
                (1i32 << 9i32 + 9i32) - 1i32,
                s!(b"functions\x00"),
            ) as *mut *mut Proto
        }
        while oldsize < (*f).sizep {
            let fresh10 = oldsize;
            oldsize = oldsize + 1;
            let ref mut fresh11 = *(*f).p.offset(fresh10 as isize);
            *fresh11 = 0 as *mut Proto
        }
    }
    let fresh12 = (*fs).np;
    (*fs).np = (*fs).np + 1;
    let ref mut fresh13 = *(*f).p.offset(fresh12 as isize);
    clp = luaF_newproto(L);
    *fresh13 = clp;
    if 0 != (*f).marked as lua_int & 1i32 << 2i32
        && 0 != (*clp).marked as lua_int & (1i32 << 0i32 | 1i32 << 1i32)
    {
        luaC_barrier_(
            L,
            &mut (*(f as *mut GCUnion)).gc,
            &mut (*(clp as *mut GCUnion)).gc,
        );
    } else {
    };
    return clp;
}
unsafe extern "C" fn getunopr(mut op: lua_int) -> UnOpr {
    match op {
        271 => return OPR_NOT,
        45 => return OPR_MINUS,
        126 => return OPR_BNOT,
        35 => return OPR_LEN,
        _ => return OPR_NOUNOPR,
    };
}
unsafe extern "C" fn enterlevel(mut ls: *mut LexState) -> () {
    let mut L: *mut lua_State = (*ls).L;
    (*L).nCcalls = (*L).nCcalls.wrapping_add(1);
    checklimit(
        (*ls).fs,
        (*L).nCcalls as lua_int,
        LUAI_MAXCCALLS,
        s!(b"C levels\x00"),
    );
}
/*
** check whether, in an assignment to an upvalue/local variable, the
** upvalue/local variable is begin used in a previous assignment to a
** table. If so, save original upvalue/local value in a safe place and
** use this safe copy in the previous assignment.
*/
unsafe extern "C" fn check_conflict(
    mut ls: *mut LexState,
    mut lh: *mut LHS_assign,
    mut v: *mut expdesc,
) -> () {
    let mut fs: *mut FuncState = (*ls).fs;
    /* eventual position to save local variable */
    let mut extra: lua_int = (*fs).freereg as lua_int;
    let mut conflict: lua_int = 0i32;
    while !lh.is_null() {
        /* check all previous assignments */
        if (*lh).v.k as lua_uint == VINDEXED as lua_int as lua_uint {
            /* assigning to a table? */
            /* table is the upvalue/local being assigned now? */
            if (*lh).v.u.ind.vt as lua_uint == (*v).k as lua_uint
                && (*lh).v.u.ind.t as lua_int == (*v).u.info
            {
                conflict = 1i32;
                (*lh).v.u.ind.vt = VLOCAL as lua_int as lu_byte;
                /* previous assignment will use safe copy */
                (*lh).v.u.ind.t = extra as lu_byte
            }
            /* index is the local being assigned? (index cannot be upvalue) */
            if (*v).k as lua_uint == VLOCAL as lua_int as lua_uint
                && (*lh).v.u.ind.idx as lua_int == (*v).u.info
            {
                conflict = 1i32;
                /* previous assignment will use safe copy */
                (*lh).v.u.ind.idx = extra as lua_short
            }
        }
        lh = (*lh).prev
    }
    if 0 != conflict {
        /* copy upvalue/local value to a temporary (in position 'extra') */
        let mut op: OpCode = (if (*v).k as lua_uint == VLOCAL as lua_int as lua_uint {
            OP_MOVE as lua_int
        } else {
            OP_GETUPVAL as lua_int
        }) as OpCode;
        luaK_codeABC(fs, op, extra, (*v).u.info, 0i32);
        luaK_reserveregs(fs, 1i32);
    };
}
unsafe extern "C" fn gotostat(mut ls: *mut LexState, mut pc: lua_int) -> () {
    let mut line: lua_int = (*ls).linenumber;
    let mut label: *mut TString = 0 as *mut TString;
    let mut g: lua_int = 0;
    if 0 != testnext(ls, TK_GOTO as lua_int) {
        label = str_checkname(ls)
    } else {
        /* skip break */
        luaX_next(ls);
        label = luaS_new((*ls).L, s!(b"break\x00"))
    }
    g = newlabelentry(ls, &mut (*(*ls).dyd).gt, label, line, pc);
    /* close it if label already defined */
    findlabel(ls, g);
}
unsafe extern "C" fn retstat(mut ls: *mut LexState) -> () {
    /* stat -> RETURN [explist] [';'] */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* registers with returned values */
    let mut first: lua_int = 0;
    let mut nret: lua_int = 0;
    if 0 != block_follow(ls, 1i32) || (*ls).t.token == ';' as i32 {
        /* return no values */
        nret = 0i32;
        first = nret
    } else {
        /* optional return values */
        nret = explist(ls, &mut e);
        if e.k as lua_uint == VCALL as lua_int as lua_uint
            || e.k as lua_uint == VVARARG as lua_int as lua_uint
        {
            luaK_setreturns(fs, &mut e, -1i32);
            if e.k as lua_uint == VCALL as lua_int as lua_uint && nret == 1i32 {
                /* tail call? */
                *(*(*fs).f).code.offset(e.u.info as isize) =
                    *(*(*fs).f).code.offset(e.u.info as isize)
                        & !(!((!(0i32 as Instruction)) << 6i32) << 0i32)
                        | (OP_TAILCALL as lua_int as Instruction) << 0i32
                            & !((!(0i32 as Instruction)) << 6i32) << 0i32
            }
            first = (*fs).nactvar as lua_int;
            /* return all values */
            nret = -1i32
        } else if nret == 1i32 {
            first = luaK_exp2anyreg(fs, &mut e)
        } else {
            /* values must go to the stack */
            luaK_exp2nextreg(fs, &mut e);
            /* return all active values */
            first = (*fs).nactvar as lua_int
        }
    }
    luaK_ret(fs, first, nret);
    /* skip optional semicolon */
    testnext(ls, ';' as i32);
}
/*============================================================*/
/* GRAMMAR RULES */
/*============================================================*/
/*
** check whether current token is in the follow set of a block.
** 'until' closes syntactical blocks, but do not close scope,
** so it is handled in separate.
*/
unsafe extern "C" fn block_follow(mut ls: *mut LexState, mut withuntil: lua_int) -> lua_int {
    match (*ls).t.token {
        260 | 261 | 262 | 289 => return 1i32,
        277 => return withuntil,
        _ => return 0i32,
    };
}
unsafe extern "C" fn labelstat(
    mut ls: *mut LexState,
    mut label: *mut TString,
    mut line: lua_int,
) -> () {
    /* label -> '::' NAME '::' */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut ll: *mut Labellist = &mut (*(*ls).dyd).label;
    /* index of new label being created */
    let mut l: lua_int = 0;
    /* check for repeated labels */
    checkrepeated(fs, ll, label);
    /* skip double colon */
    checknext(ls, TK_DBCOLON as lua_int);
    /* create new entry for this label */
    l = newlabelentry(ls, ll, label, line, luaK_getlabel(fs));
    /* skip other no-op statements */
    skipnoopstat(ls);
    if 0 != block_follow(ls, 0i32) {
        /* label is last no-op statement in the block? */
        /* assume that locals are already out of scope */
        (*(*ll).arr.offset(l as isize)).nactvar = (*(*fs).bl).nactvar
    }
    findgotos(ls, &mut *(*ll).arr.offset(l as isize));
}
/* skip no-op statements */
unsafe extern "C" fn skipnoopstat(mut ls: *mut LexState) -> () {
    while (*ls).t.token == ';' as i32 || (*ls).t.token == TK_DBCOLON as lua_int {
        statement(ls);
    }
}
/* check for repeated labels on the same block */
unsafe extern "C" fn checkrepeated(
    mut fs: *mut FuncState,
    mut ll: *mut Labellist,
    mut label: *mut TString,
) -> () {
    let mut i: lua_int = 0;
    i = (*(*fs).bl).firstlabel;
    while i < (*ll).n {
        if label == (*(*ll).arr.offset(i as isize)).name {
            let mut msg: *const lua_char = luaO_pushfstring!(
                (*(*fs).ls).L,
                s!(b"label \'%s\' already defined on line %d\x00"),
                (label as *mut lua_char)
                    .offset(::std::mem::size_of::<UTString>() as lua_ulong as isize),
                (*(*ll).arr.offset(i as isize)).line,
            );
            semerror((*fs).ls, msg);
        } else {
            i += 1
        }
    }
}
unsafe extern "C" fn localstat(mut ls: *mut LexState) -> () {
    /* stat -> LOCAL NAME {',' NAME} ['=' explist] */
    let mut nvars: lua_int = 0i32;
    let mut nexps: lua_int = 0;
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    loop {
        new_localvar(ls, str_checkname(ls));
        nvars += 1;
        if !(0 != testnext(ls, ',' as i32)) {
            break;
        }
    }
    if 0 != testnext(ls, '=' as i32) {
        nexps = explist(ls, &mut e)
    } else {
        e.k = VVOID;
        nexps = 0i32
    }
    adjust_assign(ls, nvars, nexps, &mut e);
    adjustlocalvars(ls, nvars);
}
unsafe extern "C" fn localfunc(mut ls: *mut LexState) -> () {
    let mut b: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut fs: *mut FuncState = (*ls).fs;
    /* new local variable */
    new_localvar(ls, str_checkname(ls));
    /* enter its scope */
    adjustlocalvars(ls, 1i32);
    /* function created in next register */
    body(ls, &mut b, 0i32, (*ls).linenumber);
    /* debug information will only see the variable after this point! */
    (*getlocvar(fs, b.u.info)).startpc = (*fs).pc;
}
unsafe extern "C" fn funcstat(mut ls: *mut LexState, mut line: lua_int) -> () {
    /* funcstat -> FUNCTION funcname body */
    let mut ismethod: lua_int = 0;
    let mut v: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut b: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* skip FUNCTION */
    luaX_next(ls);
    ismethod = funcname(ls, &mut v);
    body(ls, &mut b, ismethod, line);
    luaK_storevar((*ls).fs, &mut v, &mut b);
    /* definition "happens" in the first line */
    luaK_fixline((*ls).fs, line);
}
unsafe extern "C" fn funcname(mut ls: *mut LexState, mut v: *mut expdesc) -> lua_int {
    /* funcname -> NAME {fieldsel} [':' NAME] */
    let mut ismethod: lua_int = 0i32;
    singlevar(ls, v);
    while (*ls).t.token == '.' as i32 {
        fieldsel(ls, v);
    }
    if (*ls).t.token == ':' as i32 {
        ismethod = 1i32;
        fieldsel(ls, v);
    }
    return ismethod;
}
unsafe extern "C" fn repeatstat(mut ls: *mut LexState, mut line: lua_int) -> () {
    /* repeatstat -> REPEAT block UNTIL cond */
    let mut condexit: lua_int = 0;
    let mut fs: *mut FuncState = (*ls).fs;
    let mut repeat_init: lua_int = luaK_getlabel(fs);
    let mut bl1: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut bl2: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    /* loop block */
    enterblock(fs, &mut bl1, 1i32 as lu_byte);
    /* scope block */
    enterblock(fs, &mut bl2, 0i32 as lu_byte);
    /* skip REPEAT */
    luaX_next(ls);
    statlist(ls);
    check_match(ls, TK_UNTIL as lua_int, TK_REPEAT as lua_int, line);
    /* read condition (inside scope block) */
    condexit = cond(ls);
    /* upvalues? */
    if 0 != bl2.upval {
        luaK_patchclose(fs, condexit, bl2.nactvar as lua_int);
    }
    /* finish scope */
    leaveblock(fs);
    /* close the loop */
    luaK_patchlist(fs, condexit, repeat_init);
    /* finish loop */
    leaveblock(fs);
}
unsafe extern "C" fn cond(mut ls: *mut LexState) -> lua_int {
    /* cond -> exp */
    let mut v: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* read condition */
    expr(ls, &mut v);
    if v.k as lua_uint == VNIL as lua_int as lua_uint {
        /* 'falses' are all equal here */
        v.k = VFALSE
    }
    luaK_goiftrue((*ls).fs, &mut v);
    return v.f;
}
unsafe extern "C" fn forstat(mut ls: *mut LexState, mut line: lua_int) -> () {
    /* forstat -> FOR (fornum | forlist) END */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut varname: *mut TString = 0 as *mut TString;
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    /* scope for loop and control variables */
    enterblock(fs, &mut bl, 1i32 as lu_byte);
    /* skip 'for' */
    luaX_next(ls);
    /* first variable name */
    varname = str_checkname(ls);
    match (*ls).t.token {
        61 => {
            fornum(ls, varname, line);
        }
        44 | 268 => {
            forlist(ls, varname);
        }
        _ => {
            luaX_syntaxerror(ls, s!(b"\'=\' or \'in\' expected\x00"));
        }
    }
    check_match(ls, TK_END as lua_int, TK_FOR as lua_int, line);
    /* loop scope ('break' jumps to this point) */
    leaveblock(fs);
}
unsafe extern "C" fn forlist(mut ls: *mut LexState, mut indexname: *mut TString) -> () {
    /* forlist -> NAME {,NAME} IN explist forbody */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* gen, state, control, plus at least one declared var */
    let mut nvars: lua_int = 4i32;
    let mut line: lua_int = 0;
    let mut base: lua_int = (*fs).freereg as lua_int;
    /* create control variables */
    new_localvarliteral_(
        ls,
        s!(b"(for generator)\x00"),
        (::std::mem::size_of::<[lua_char; 16]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    new_localvarliteral_(
        ls,
        s!(b"(for state)\x00"),
        (::std::mem::size_of::<[lua_char; 12]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    new_localvarliteral_(
        ls,
        s!(b"(for control)\x00"),
        (::std::mem::size_of::<[lua_char; 14]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    /* create declared variables */
    new_localvar(ls, indexname);
    while 0 != testnext(ls, ',' as i32) {
        new_localvar(ls, str_checkname(ls));
        nvars += 1
    }
    checknext(ls, TK_IN as lua_int);
    line = (*ls).linenumber;
    adjust_assign(ls, 3i32, explist(ls, &mut e), &mut e);
    /* extra space to call generator */
    luaK_checkstack(fs, 3i32);
    forbody(ls, base, line, nvars - 3i32, 0i32);
}
unsafe extern "C" fn forbody(
    mut ls: *mut LexState,
    mut base: lua_int,
    mut line: lua_int,
    mut nvars: lua_int,
    mut isnum: lua_int,
) -> () {
    /* forbody -> DO block */
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut fs: *mut FuncState = (*ls).fs;
    let mut prep: lua_int = 0;
    let mut endfor: lua_int = 0;
    /* control variables */
    adjustlocalvars(ls, 3i32);
    checknext(ls, TK_DO as lua_int);
    prep = if 0 != isnum {
        luaK_codeABx(
            fs,
            OP_FORPREP,
            base,
            (-1i32 + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as lua_uint,
        )
    } else {
        luaK_jump(fs)
    };
    /* scope for declared variables */
    enterblock(fs, &mut bl, 0i32 as lu_byte);
    adjustlocalvars(ls, nvars);
    luaK_reserveregs(fs, nvars);
    block(ls);
    /* end of scope for declared variables */
    leaveblock(fs);
    luaK_patchtohere(fs, prep);
    /* numeric for? */
    if 0 != isnum {
        endfor = luaK_codeABx(
            fs,
            OP_FORLOOP,
            base,
            (-1i32 + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as lua_uint,
        )
    } else {
        /* generic for */
        luaK_codeABC(fs, OP_TFORCALL, base, 0i32, nvars);
        luaK_fixline(fs, line);
        endfor = luaK_codeABx(
            fs,
            OP_TFORLOOP,
            base + 2i32,
            (-1i32 + ((1i32 << 9i32 + 9i32) - 1i32 >> 1i32)) as lua_uint,
        )
    }
    luaK_patchlist(fs, endfor, prep + 1i32);
    luaK_fixline(fs, line);
}
/* }==================================================================== */
/*
** {======================================================================
** Rules for Statements
** =======================================================================
*/
unsafe extern "C" fn block(mut ls: *mut LexState) -> () {
    /* block -> statlist */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    enterblock(fs, &mut bl, 0i32 as lu_byte);
    statlist(ls);
    leaveblock(fs);
}
unsafe extern "C" fn fornum(
    mut ls: *mut LexState,
    mut varname: *mut TString,
    mut line: lua_int,
) -> () {
    /* fornum -> NAME = exp1,exp1[,exp1] forbody */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut base: lua_int = (*fs).freereg as lua_int;
    new_localvarliteral_(
        ls,
        s!(b"(for index)\x00"),
        (::std::mem::size_of::<[lua_char; 12]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    new_localvarliteral_(
        ls,
        s!(b"(for limit)\x00"),
        (::std::mem::size_of::<[lua_char; 12]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    new_localvarliteral_(
        ls,
        s!(b"(for step)\x00"),
        (::std::mem::size_of::<[lua_char; 11]>() as lua_ulong)
            .wrapping_div(::std::mem::size_of::<lua_char>() as lua_ulong)
            .wrapping_sub(1i32 as lua_ulong),
    );
    new_localvar(ls, varname);
    checknext(ls, '=' as i32);
    /* initial value */
    exp1(ls);
    checknext(ls, ',' as i32);
    /* limit */
    exp1(ls);
    if 0 != testnext(ls, ',' as i32) {
        /* optional step */
        exp1(ls);
    } else {
        /* default step = 1 */
        luaK_codek(
            fs,
            (*fs).freereg as lua_int,
            luaK_intK(fs, 1i32 as lua_Integer),
        );
        luaK_reserveregs(fs, 1i32);
    }
    forbody(ls, base, line, 1i32, 1i32);
}
unsafe extern "C" fn exp1(mut ls: *mut LexState) -> lua_int {
    let mut e: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    let mut reg: lua_int = 0;
    expr(ls, &mut e);
    luaK_exp2nextreg((*ls).fs, &mut e);
    reg = e.u.info;
    return reg;
}
unsafe extern "C" fn whilestat(mut ls: *mut LexState, mut line: lua_int) -> () {
    /* whilestat -> WHILE cond DO block END */
    let mut fs: *mut FuncState = (*ls).fs;
    let mut whileinit: lua_int = 0;
    let mut condexit: lua_int = 0;
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    /* skip WHILE */
    luaX_next(ls);
    whileinit = luaK_getlabel(fs);
    condexit = cond(ls);
    enterblock(fs, &mut bl, 1i32 as lu_byte);
    checknext(ls, TK_DO as lua_int);
    block(ls);
    luaK_patchlist(fs, luaK_jump(fs), whileinit);
    check_match(ls, TK_END as lua_int, TK_WHILE as lua_int, line);
    leaveblock(fs);
    /* false conditions finish the loop */
    luaK_patchtohere(fs, condexit);
}
unsafe extern "C" fn ifstat(mut ls: *mut LexState, mut line: lua_int) -> () {
    /* ifstat -> IF cond THEN block {ELSEIF cond THEN block} [ELSE block] END */
    let mut fs: *mut FuncState = (*ls).fs;
    /* exit list for finished parts */
    let mut escapelist: lua_int = -1i32;
    /* IF cond THEN block */
    test_then_block(ls, &mut escapelist);
    while (*ls).t.token == TK_ELSEIF as lua_int {
        /* ELSEIF cond THEN block */
        test_then_block(ls, &mut escapelist);
    }
    if 0 != testnext(ls, TK_ELSE as lua_int) {
        /* 'else' part */
        block(ls);
    }
    check_match(ls, TK_END as lua_int, TK_IF as lua_int, line);
    /* patch escape list to 'if' end */
    luaK_patchtohere(fs, escapelist);
}
unsafe extern "C" fn test_then_block(mut ls: *mut LexState, mut escapelist: *mut lua_int) -> () {
    /* test_then_block -> [IF | ELSEIF] cond THEN block */
    let mut bl: BlockCnt = BlockCnt {
        previous: 0 as *mut BlockCnt,
        firstlabel: 0,
        firstgoto: 0,
        nactvar: 0,
        upval: 0,
        isloop: 0,
    };
    let mut fs: *mut FuncState = (*ls).fs;
    let mut v: expdesc = expdesc {
        k: VVOID,
        u: expdesc_1 { ival: 0 },
        t: 0,
        f: 0,
    };
    /* instruction to skip 'then' code (if condition is false) */
    let mut jf: lua_int = 0;
    /* skip IF or ELSEIF */
    luaX_next(ls);
    /* read condition */
    expr(ls, &mut v);
    checknext(ls, TK_THEN as lua_int);
    if (*ls).t.token == TK_GOTO as lua_int || (*ls).t.token == TK_BREAK as lua_int {
        /* will jump to label if condition is true */
        luaK_goiffalse((*ls).fs, &mut v);
        /* must enter block before 'goto' */
        enterblock(fs, &mut bl, 0i32 as lu_byte);
        /* handle goto/break */
        gotostat(ls, v.t);
        while 0 != testnext(ls, ';' as i32) {}
        /* skip colons */
        if 0 != block_follow(ls, 0i32) {
            /* 'goto' is the entire block? */
            leaveblock(fs);
            /* and that is it */
            return;
        } else {
            jf = luaK_jump(fs)
        }
    } else {
        /* regular case (not goto/break) */
        /* skip over block if condition is false */
        luaK_goiftrue((*ls).fs, &mut v);
        enterblock(fs, &mut bl, 0i32 as lu_byte);
        jf = v.f
    }
    /* 'then' part */
    statlist(ls);
    leaveblock(fs);
    if (*ls).t.token == TK_ELSE as lua_int || (*ls).t.token == TK_ELSEIF as lua_int {
        /* followed by 'else'/'elseif'? */
        /* must jump over it */
        luaK_concat(fs, escapelist, luaK_jump(fs));
    }
    luaK_patchtohere(fs, jf);
}
