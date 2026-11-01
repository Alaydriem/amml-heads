
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_146bdec10(longlong param_1,undefined8 param_2,longlong param_3)

{
  undefined8 *puVar1;
  int *piVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong lVar5;
  char cVar6;
  undefined8 uVar7;
  longlong lVar8;
  longlong lVar9;
  char *pcStack_50;
  longlong lStack_48;
  longlong lStack_40;
  
  if (((*(char **)(param_1 + 0x2b0) != (char *)0x0) && (**(char **)(param_1 + 0x2b0) == '\x01')) &&
     (cVar6 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(), cVar6 == '\0')) {
    puVar1 = (undefined8 *)(param_3 + 0x40);
    cVar6 = FUN_146bdee80(puVar1);
    if (cVar6 != '\0') {
      cVar6 = FUN_146bc4800(param_1,param_3 + 0x30,puVar1);
      if (cVar6 != '\0') {
        if (*(char **)(param_1 + 0x2b0) == (char *)0x0) {
          _DAT_00000000 = 0xdeadc0de;
        }
        if (**(char **)(param_1 + 0x2b0) == '\0') {
          _DAT_00000000 = 0xdeadc0de;
        }
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(*(undefined8 *)(param_1 + 0x2c0),&pcStack_50)
        ;
        lVar5 = lStack_48;
        if (pcStack_50 == (char *)0x0) {
          _DAT_00000000 = 0xdeadc0de;
        }
        if (*pcStack_50 == '\0') {
          _DAT_00000000 = 0xdeadc0de;
        }
        lVar3 = *(longlong *)(lStack_40 + 8);
        lVar8 = ((*(longlong *)(param_3 + 0x38) * 0x1f1f1f1f ^ *(ulonglong *)(param_3 + 0x30)) &
                *(ulonglong *)(lStack_40 + 0x30)) * 0x10;
        lVar9 = *(longlong *)(*(longlong *)(lStack_40 + 0x18) + 8 + lVar8);
        if (lVar9 != lVar3) {
          uVar4 = *(ulonglong *)(lVar9 + 0x10);
          while( true ) {
            if ((*(ulonglong *)(param_3 + 0x30) == uVar4) &&
               (*(longlong *)(param_3 + 0x38) == *(longlong *)(lVar9 + 0x18))) goto LAB_146bded48;
            if (lVar9 == *(longlong *)(*(longlong *)(lStack_40 + 0x18) + lVar8)) break;
            lVar9 = *(longlong *)(lVar9 + 8);
            uVar4 = *(ulonglong *)(lVar9 + 0x10);
          }
        }
        lVar9 = 0;
LAB_146bded48:
        if (lVar9 == 0) {
          lVar9 = lVar3;
        }
        pcStack_50 = (char *)0x0;
        lStack_48 = 0;
        if (lVar5 != 0) {
          LOCK();
          piVar2 = (int *)(lVar5 + 8);
          *piVar2 = *piVar2 + -1;
          UNLOCK();
          if (*piVar2 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5);
            LOCK();
            piVar2 = (int *)(lVar5 + 0xc);
            *piVar2 = *piVar2 + -1;
            UNLOCK();
            if (*piVar2 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5);
            }
          }
          lVar5 = lStack_48;
          lStack_40 = 0;
          if (lStack_48 != 0) {
            LOCK();
            piVar2 = (int *)(lStack_48 + 8);
            *piVar2 = *piVar2 + -1;
            UNLOCK();
            if (*piVar2 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_48);
              LOCK();
              piVar2 = (int *)(lVar5 + 0xc);
              *piVar2 = *piVar2 + -1;
              UNLOCK();
              if (*piVar2 == 0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5);
              }
            }
          }
        }
        if (lVar9 != lVar3) {
          if (*(longlong *)(param_3 + 0x48) == 0) {
            uVar7 = 0;
          }
          else {
            LOCK();
            piVar2 = (int *)(*(longlong *)(param_3 + 0x48) + 8);
            *piVar2 = *piVar2 + 1;
            UNLOCK();
            uVar7 = *(undefined8 *)(param_3 + 0x48);
          }
          *(undefined8 *)(lVar9 + 0xc0) = *puVar1;
          lVar5 = *(longlong *)(lVar9 + 200);
          *(undefined8 *)(lVar9 + 200) = uVar7;
          if (lVar5 != 0) {
            LOCK();
            piVar2 = (int *)(lVar5 + 8);
            *piVar2 = *piVar2 + -1;
            UNLOCK();
            if (*piVar2 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5);
              LOCK();
              piVar2 = (int *)(lVar5 + 0xc);
              *piVar2 = *piVar2 + -1;
              UNLOCK();
              if (*piVar2 == 0) {
                (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5);
              }
            }
          }
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)
                    (*(undefined8 *)(param_1 + 0x3f8),param_3 + 0x30,puVar1,lVar9 + 0x78,1);
        }
      }
    }
    *(undefined1 *)(param_3 + 0x11) = 1;
  }
  return;
}

