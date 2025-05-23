default Order dec
$include <prelude.sail>

// ——————————————————————————————— Arch size ———————————————————————————————— //

type xlen : Int = 64
type xlen_bytes : Int = 8
type xlenbits = bits(xlen)

// ——————————————————————————————— Extensions ——————————————————————————————— //

scattered enum extension

val hartSupports : extension -> bool
scattered function hartSupports

enum clause extension = Ext_U
function clause hartSupports(Ext_U) = config extensions.U.supported

enum clause extension = Ext_S
function clause hartSupports(Ext_S) = config extensions.S.supported

// ——————————————————————————————— Registers ———————————————————————————————— //

type regbits = bits(5)

// Program counter
register PC : xlenbits

// —————————————————————————————— Entry point ——————————————————————————————— //

scattered union ast

val execute : ast -> unit
union clause ast = TEST : unit

function clause execute TEST() = {
    if hartSupports(Ext_U) then
        // Something
        ()
}
