
undefined8 * FUN_143cbf1f0(longlong param_1,undefined8 *param_2)

{
  int *piVar1;
  ulonglong uVar2;
  longlong lVar3;
  undefined4 uVar4;
  undefined4 uVar5;
  undefined4 uVar6;
  undefined8 uVar7;
  undefined8 *puVar8;
  undefined8 *puVar9;
  
  FUN_1426478e0(param_2);
  uVar4 = *(undefined4 *)(param_1 + 0xc);
  uVar5 = *(undefined4 *)(param_1 + 0x10);
  uVar6 = *(undefined4 *)(param_1 + 0x14);
  *(undefined4 *)(param_2 + 1) = *(undefined4 *)(param_1 + 8);
  *(undefined4 *)((longlong)param_2 + 0xc) = uVar4;
  *(undefined4 *)(param_2 + 2) = uVar5;
  *(undefined4 *)((longlong)param_2 + 0x14) = uVar6;
  *param_2 = *(undefined8 *)(param_1 + 0x18);
  puVar9 = (undefined8 *)(param_1 + 0x20);
  puVar8 = param_2 + 3;
  if (puVar8 != puVar9) {
    uVar2 = *(ulonglong *)(param_1 + 0x30);
    if (0xf < *(ulonglong *)(param_1 + 0x38)) {
      puVar9 = *(undefined8 **)(param_1 + 0x20);
    }
    if ((ulonglong)param_2[6] < uVar2) {
      FUN_14009daf0(puVar8,uVar2);
    }
    else {
      if (0xf < (ulonglong)param_2[6]) {
        puVar8 = (undefined8 *)param_2[3];
      }
      param_2[5] = uVar2;
      FUN_14e5a62c0(puVar8,puVar9,uVar2);
      *(undefined1 *)((longlong)puVar8 + uVar2) = 0;
    }
  }
  puVar9 = (undefined8 *)(param_1 + 0x40);
  puVar8 = param_2 + 7;
  if (puVar8 != puVar9) {
    uVar2 = *(ulonglong *)(param_1 + 0x50);
    if (0xf < *(ulonglong *)(param_1 + 0x58)) {
      puVar9 = *(undefined8 **)(param_1 + 0x40);
    }
    if ((ulonglong)param_2[10] < uVar2) {
      FUN_14009daf0(puVar8,uVar2);
    }
    else {
      if (0xf < (ulonglong)param_2[10]) {
        puVar8 = (undefined8 *)param_2[7];
      }
      param_2[9] = uVar2;
      FUN_14e5a62c0(puVar8,puVar9,uVar2);
      *(undefined1 *)((longlong)puVar8 + uVar2) = 0;
    }
  }
  puVar9 = (undefined8 *)(param_1 + 0x60);
  puVar8 = param_2 + 0xb;
  if (puVar8 != puVar9) {
    uVar2 = *(ulonglong *)(param_1 + 0x70);
    if (0xf < *(ulonglong *)(param_1 + 0x78)) {
      puVar9 = *(undefined8 **)(param_1 + 0x60);
    }
    if ((ulonglong)param_2[0xe] < uVar2) {
      FUN_14009daf0(puVar8,uVar2);
    }
    else {
      if (0xf < (ulonglong)param_2[0xe]) {
        puVar8 = (undefined8 *)param_2[0xb];
      }
      param_2[0xd] = uVar2;
      FUN_14e5a62c0(puVar8,puVar9,uVar2);
      *(undefined1 *)((longlong)puVar8 + uVar2) = 0;
    }
  }
  *(undefined4 *)(param_2 + 0xf) = *(undefined4 *)(param_1 + 0x80);
  if (*(longlong *)(param_1 + 0x90) == 0) {
    uVar7 = 0;
  }
  else {
    LOCK();
    piVar1 = (int *)(*(longlong *)(param_1 + 0x90) + 8);
    *piVar1 = *piVar1 + 1;
    UNLOCK();
    uVar7 = *(undefined8 *)(param_1 + 0x90);
  }
  param_2[0x10] = *(undefined8 *)(param_1 + 0x88);
  lVar3 = param_2[0x11];
  param_2[0x11] = uVar7;
  if (lVar3 != 0) {
    LOCK();
    piVar1 = (int *)(lVar3 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar3);
      LOCK();
      piVar1 = (int *)(lVar3 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar3);
      }
    }
  }
  *(undefined1 *)(param_2 + 0x14) = *(undefined1 *)(param_1 + 0x98);
  *(undefined1 *)((longlong)param_2 + 0xa1) = *(undefined1 *)(param_1 + 0x99);
  *(undefined1 *)((longlong)param_2 + 0xa2) = *(undefined1 *)(param_1 + 0x9a);
  uVar7 = *(undefined8 *)(param_1 + 0xa4);
  param_2[0x12] = *(undefined8 *)(param_1 + 0x9c);
  param_2[0x13] = uVar7;
  return param_2;
}

