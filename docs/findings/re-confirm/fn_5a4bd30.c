
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

undefined8 * FUN_145a4bd30(undefined8 *param_1,undefined8 *param_2)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 uVar3;
  undefined1 uVar4;
  undefined4 uVar5;
  undefined8 unaff_RBX;
  
  *param_1 = FUN_1403c7b10;
  param_1[1] = 0;
  param_1[2] = 0;
  *(undefined8 *)((longlong)param_1 + 0x17) = 0;
  *(undefined8 *)((longlong)param_1 + 0x1f) = 0;
  *(undefined4 *)(param_1 + 5) = 1;
  uVar4 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1b0,0x53);
  uVar5 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1b0,0x53);
  if (param_2 != param_1) {
    lVar1 = param_2[1];
    param_2[1] = 0;
    lVar2 = param_1[1];
    param_1[1] = lVar1;
    if (lVar2 != 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    }
    *param_1 = *param_2;
    param_1[2] = param_2[2];
    param_2[2] = 0;
    *(undefined4 *)(param_1 + 5) = *(undefined4 *)(param_2 + 5);
    uVar3 = param_2[4];
    param_1[3] = param_2[3];
    param_1[4] = uVar3;
    param_2[3] = 0;
    param_2[4] = 0;
    *(undefined4 *)(param_2 + 5) = 1;
  }
  (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1b0,uVar5);
  (*(code *)PTR__guard_dispatch_icall_14f99ef70)
            (_DAT_15233c1b0,CONCAT71((int7)((ulonglong)unaff_RBX >> 8),uVar4) & 0xffffffff);
  return param_1;
}

