
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_1454239a0(longlong param_1,longlong param_2,undefined4 param_3,undefined4 param_4)

{
  int *piVar1;
  longlong lStack_110;
  longlong lStack_108;
  undefined1 auStack_b8 [8];
  longlong lStack_b0;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined8 uStack_80;
  undefined8 uStack_78;
  undefined8 uStack_70;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined1 uStack_54;
  undefined4 uStack_50;
  undefined1 uStack_4c;
  longlong lStack_48;
  undefined8 uStack_40;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  FUN_1454756c0(param_1 + 0x1a8,&lStack_110,param_2);
  uStack_40 = *(undefined8 *)(param_2 + 0x18);
  if (*(ulonglong *)(param_2 + 0x20) < 0x10) {
    lStack_48 = param_2 + 8;
  }
  else {
    lStack_48 = *(longlong *)(param_2 + 8);
  }
  uStack_80 = _DAT_14f249600;
  uStack_78 = _UNK_14f249608;
  uStack_70 = 1;
  uStack_68 = _DAT_14f0feae0;
  uStack_64 = _UNK_14f0feae4;
  uStack_60 = _UNK_14f0feae8;
  uStack_5c = _UNK_14f0feaec;
  uStack_58 = 0;
  uStack_54 = 0;
  uStack_50 = 8;
  uStack_4c = 0;
  uStack_88 = param_3;
  uStack_84 = param_4;
  FUN_145a4bb80(auStack_b8,&uStack_88);
  FUN_144df8eb0(&lStack_110,auStack_b8,1);
  FUN_145673bb0(lStack_110 + 0x58,*(undefined8 *)(param_1 + 0x1d0),&lStack_110,1,&lStack_48,0);
  if (lStack_108 != 0) {
    LOCK();
    piVar1 = (int *)(lStack_108 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_108);
      LOCK();
      piVar1 = (int *)(lStack_108 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_108);
      }
    }
  }
  if (lStack_b0 != 0) {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  }
  return;
}

