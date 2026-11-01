
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14e932900(undefined8 param_1,undefined8 param_2)

{
  undefined1 auStack_b8 [40];
  undefined8 uStack_90;
  undefined *puStack_88;
  undefined1 auStack_80 [24];
  undefined1 auStack_68 [32];
  undefined1 auStack_48 [48];
  ulonglong uStack_18;
  
  uStack_18 = _DAT_1523080c0 ^ (ulonglong)auStack_b8;
  uStack_90 = param_2;
  FUN_14e947550(&puStack_88);
  FUN_14e9475b0(&puStack_88,param_2,param_1);
  puStack_88 = &UNK_14eec7050;
  FUN_14e9305b0(auStack_48);
  FUN_14e9305b0(auStack_68);
  FUN_14e933cc0(auStack_80);
  FUN_14e946d20(&puStack_88);
  FUN_14e950950(uStack_18 ^ (ulonglong)auStack_b8);
  return;
}

