
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

undefined8 FUN_1457354b0(undefined8 param_1,longlong param_2,char param_3)

{
  int *piVar1;
  char cVar2;
  int iVar3;
  int iVar4;
  longlong *plVar5;
  longlong lVar6;
  float extraout_XMM0_Da;
  float fVar7;
  char *pcStack_40;
  longlong lStack_38;
  longlong lStack_30;
  
  plVar5 = (longlong *)(*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  cVar2 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(plVar5,*(undefined8 *)(*plVar5 + 0x2f8));
  if (cVar2 == '\0') {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(param_1,&pcStack_40);
    lVar6 = lStack_38;
    if (pcStack_40 == (char *)0x0) {
      _DAT_00000000 = 0xdeadc0de;
    }
    else if (*pcStack_40 == '\0') {
      _DAT_00000000 = 0xdeadc0de;
    }
    iVar3 = *(int *)(lStack_30 + 0x454);
    pcStack_40 = (char *)0x0;
    lStack_38 = 0;
    if (lVar6 != 0) {
      LOCK();
      piVar1 = (int *)(lVar6 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar6);
        LOCK();
        piVar1 = (int *)(lVar6 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar6);
        }
      }
      lVar6 = lStack_38;
      lStack_30 = 0;
      if (lStack_38 != 0) {
        LOCK();
        piVar1 = (int *)(lStack_38 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_38);
          LOCK();
          piVar1 = (int *)(lVar6 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar6);
          }
        }
      }
    }
    if ((iVar3 != 0) && (lVar6 = func_0x000140b9d8f0(param_2), lVar6 != 0)) {
      cVar2 = func_0x000143d56740(lVar6 + 8,0x2e);
      if (cVar2 != '\0') {
        FUN_14297d990(param_2 + 8);
        if (extraout_XMM0_Da <= _DAT_14eec7060) {
          fVar7 = extraout_XMM0_Da;
          if (extraout_XMM0_Da <= 0.0) {
            fVar7 = 0.0;
          }
          if (fVar7 <= 0.0) {
            iVar3 = func_0x00014297c120(param_2 + 8);
            if (iVar3 != -1) {
              iVar4 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
              if ((param_3 == '\0') && (iVar3 == 1 || iVar4 == 1 && iVar3 == 5)) {
                return 1;
              }
            }
            cVar2 = func_0x000143d56740(lVar6 + 8,0x6c);
            if (cVar2 == '\0') {
              return 0;
            }
          }
        }
        return 1;
      }
    }
  }
  return 0;
}

