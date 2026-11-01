
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

undefined8 FUN_145741750(undefined8 param_1,longlong param_2,char param_3)

{
  int *piVar1;
  longlong lVar2;
  char cVar3;
  int iVar4;
  int iVar5;
  longlong *plVar6;
  undefined8 uVar7;
  float fVar8;
  char *pcStack_38;
  longlong lStack_30;
  longlong lStack_28;
  
  plVar6 = (longlong *)(*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  cVar3 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(plVar6,*(undefined8 *)(*plVar6 + 0x2f8));
  if (cVar3 == '\0') {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_1,&pcStack_38);
    lVar2 = lStack_30;
    if (pcStack_38 == (char *)0x0) {
      _DAT_00000000 = 0xdeadc0de;
    }
    else if (*pcStack_38 == '\0') {
      _DAT_00000000 = 0xdeadc0de;
    }
    iVar4 = *(int *)(lStack_28 + 0x454);
    pcStack_38 = (char *)0x0;
    lStack_30 = 0;
    if (lVar2 != 0) {
      LOCK();
      piVar1 = (int *)(lVar2 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar2);
        LOCK();
        piVar1 = (int *)(lVar2 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar2);
        }
      }
      lVar2 = lStack_30;
      lStack_28 = 0;
      if (lStack_30 != 0) {
        LOCK();
        piVar1 = (int *)(lStack_30 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_30);
          LOCK();
          piVar1 = (int *)(lVar2 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar2);
          }
        }
      }
    }
    if (iVar4 != 0) {
      fVar8 = (float)FUN_14297d990(param_2 + 8);
      if (fVar8 <= _DAT_14eec7060) {
        if (fVar8 <= 0.0) {
          fVar8 = 0.0;
        }
        if (fVar8 <= 0.0) {
          iVar4 = func_0x00014297c120(param_2 + 8);
          if (iVar4 == -1) {
            return 0;
          }
          iVar5 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
          if (param_3 != '\0') {
            return 0;
          }
          if (iVar4 != 1 && (iVar5 != 1 || iVar4 != 5)) {
            return 0;
          }
        }
      }
      plVar6 = (longlong *)func_0x000140b9d8f0(param_2);
      if (plVar6 != (longlong *)0x0) {
        uVar7 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                          (plVar6,*(undefined8 *)(*plVar6 + 0x1d8));
        return uVar7;
      }
    }
  }
  return 0;
}

