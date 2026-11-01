
void FUN_1425147e0(longlong param_1,longlong param_2)

{
  int *piVar1;
  int *piVar2;
  uint uVar3;
  longlong lVar4;
  longlong lVar5;
  undefined4 uVar6;
  longlong lVar7;
  ulonglong uVar8;
  longlong *plVar9;
  undefined8 uVar10;
  undefined8 uStack_50;
  undefined4 uStack_48;
  undefined4 uStack_44;
  uint uStack_40;
  char cStack_38;
  
  if (*(char *)(param_1 + 0x60) != '\x01') {
    return;
  }
  if (*(char *)(param_1 + 0x61) == '\0') {
    return;
  }
  uVar10 = *(undefined8 *)(param_2 + 0x200);
  FUN_142b96a10(&uStack_50,param_1 + 0x80);
  if (cStack_38 == '\x01') {
    lVar4 = CONCAT44(uStack_44,uStack_48);
    plVar9 = (longlong *)
             (*(longlong *)(lVar4 + 0x48) +
             (ulonglong)
             ((int)((ulonglong)(*(longlong *)(lVar4 + 0x50) - *(longlong *)(lVar4 + 0x48)) >> 3) -
              1U & 0x85b93800) * 8);
    lVar5 = *(longlong *)(lVar4 + 0x68);
    do {
      if (*plVar9 == -1) goto LAB_1425148f8;
      lVar7 = *plVar9 * 0x20;
      plVar9 = (longlong *)(lVar5 + lVar7);
    } while (*(int *)(lVar5 + 8 + lVar7) != -0x7a46c800);
    if ((*(longlong *)(lVar4 + 0x70) != lVar5 + lVar7) &&
       (lVar5 = *(longlong *)(lVar5 + lVar7 + 0x10), lVar5 != 0)) {
      uVar8 = (ulonglong)((uStack_40 & 0x3ffff) >> 0xb);
      if (((uVar8 < (ulonglong)(*(longlong *)(lVar5 + 0x10) - *(longlong *)(lVar5 + 8) >> 3)) &&
          (((lVar4 = *(longlong *)(*(longlong *)(lVar5 + 8) + uVar8 * 8), lVar4 != 0 &&
            (uVar3 = *(uint *)(lVar4 + (ulonglong)(uStack_40 & 0x7ff) * 4),
            (uStack_40 & 0xfffc0000 ^ uVar3) < 0x3ffff)) &&
           (lVar5 = *(longlong *)(*(longlong *)(lVar5 + 0x50) + (ulonglong)(uVar3 >> 4 & 0x3ff8)),
           lVar5 != 0)))) &&
         (lVar5 = *(longlong *)(lVar5 + (ulonglong)(uVar3 & 0x7f) * 8),
         *(char *)(lVar5 + 0x289) == '\0')) goto LAB_1425149a8;
    }
  }
LAB_1425148f8:
  lVar5 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uVar10,*(undefined8 *)(param_1 + 0x78),0);
  if (lVar5 == 0) {
    return;
  }
  lVar4 = (*(undefined8 **)(lVar5 + 8))[1];
  if (lVar4 == 0) {
    uVar6 = *(undefined4 *)(lVar5 + 0x18);
    uVar10 = 0;
  }
  else {
    uVar10 = **(undefined8 **)(lVar5 + 8);
    LOCK();
    *(int *)(lVar4 + 0xc) = *(int *)(lVar4 + 0xc) + 1;
    UNLOCK();
    LOCK();
    *(int *)(lVar4 + 0xc) = *(int *)(lVar4 + 0xc) + 1;
    UNLOCK();
    piVar1 = (int *)(lVar4 + 0xc);
    LOCK();
    piVar2 = (int *)(lVar4 + 0xc);
    *piVar2 = *piVar2 + -1;
    UNLOCK();
    if (*piVar2 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar4);
    }
    uVar6 = *(undefined4 *)(lVar5 + 0x18);
    LOCK();
    *piVar1 = *piVar1 + 1;
    UNLOCK();
    LOCK();
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar4);
    }
  }
  *(undefined8 *)(param_1 + 0x80) = uVar10;
  lVar7 = *(longlong *)(param_1 + 0x88);
  *(longlong *)(param_1 + 0x88) = lVar4;
  if (lVar7 != 0) {
    LOCK();
    piVar1 = (int *)(lVar7 + 0xc);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 == 0) {
      (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    }
  }
  *(undefined4 *)(param_1 + 0x90) = uVar6;
LAB_1425149a8:
  uStack_48 = *(undefined4 *)(*(undefined8 **)(lVar5 + 0x240) + 1);
  uStack_50 = **(undefined8 **)(lVar5 + 0x240);
  FUN_140b87fc0(lVar5,&uStack_44);
  *(undefined8 *)(param_1 + 8) = uStack_50;
  *(ulonglong *)(param_1 + 0x10) = CONCAT44(uStack_44,uStack_48);
  return;
}

