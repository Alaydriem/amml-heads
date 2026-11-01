
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

void FUN_14541b650(longlong param_1,undefined8 param_2,undefined8 param_3)

{
  code *pcVar1;
  undefined1 auStack_48 [40];
  longlong alStack_20 [2];
  ulonglong uStack_10;
  
  uStack_10 = _DAT_1523080c0 ^ (ulonglong)auStack_48;
  if (*(char **)(param_1 + 8) == (char *)0x0) {
    _DAT_00000000 = 0xdeadc0de;
  }
  else if (**(char **)(param_1 + 8) == '\0') {
    _DAT_00000000 = 0xdeadc0de;
  }
  FUN_1454756c0(*(longlong *)(param_1 + 0x18) + 0x1a8,alStack_20,param_2);
  FUN_145676a20(alStack_20[0] + 0x58,param_3,0);
  if (_DAT_1523080c0 != (uStack_10 ^ (ulonglong)auStack_48)) {
    FUN_14e950950(uStack_10 ^ (ulonglong)auStack_48);
    pcVar1 = (code *)swi(3);
    (*pcVar1)();
    return;
  }
  return;
}

