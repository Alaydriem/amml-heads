
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14134a4c0(longlong param_1,undefined8 param_2,longlong param_3)

{
  undefined8 *puVar1;
  int *piVar2;
  longlong lVar3;
  ulonglong uVar4;
  longlong lVar5;
  char cVar6;
  undefined8 uVar7;
  ulonglong uVar8;
  longlong lVar9;
  char *pcStack_60;
  longlong lStack_58;
  longlong lStack_50;
  undefined8 uStack_48;
  
  uStack_48 = 0xfffffffffffffffe;
  if (((*(char **)(param_1 + 600) != (char *)0x0) && (**(char **)(param_1 + 600) == '\x01')) &&
     (cVar6 = (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(), cVar6 == '\0')) {
    puVar1 = (undefined8 *)(param_3 + 0x40);
    cVar6 = FUN_14134a830(puVar1);
    if (cVar6 != '\0') {
      cVar6 = FUN_14132de40(param_1,param_3 + 0x30,puVar1);
      if (cVar6 != '\0') {
        if ((*(longlong *)(param_1 + 600) == 0) &&
           (cVar6 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed61d45),
           cVar6 != '\0')) {
          _DAT_00000000 = 0xdeadc0de;
        }
        if ((**(char **)(param_1 + 600) == '\0') &&
           (cVar6 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed61d45)
           , cVar6 != '\0')) {
          _DAT_00000000 = 0xdeadc0de;
        }
        (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(*(undefined8 *)(param_1 + 0x268),&pcStack_60)
        ;
        if ((pcStack_60 == (char *)0x0) &&
           (cVar6 = FUN_140289fe0(&UNK_14ec0360b,&UNK_14ec4122d,300,&UNK_14ec489d8,&UNK_14ed315bc),
           cVar6 != '\0')) {
          _DAT_00000000 = 0xdeadc0de;
        }
        if ((*pcStack_60 == '\0') &&
           (cVar6 = FUN_140289fe0(&UNK_14ec89e5e,&UNK_14ec87783,0x130,&UNK_14ec489d8,&UNK_14ed315bc)
           , cVar6 != '\0')) {
          _DAT_00000000 = 0xdeadc0de;
        }
        lVar5 = lStack_58;
        uVar8 = (*(longlong *)(param_3 + 0x38) * 0x1f1f1f1f ^ *(ulonglong *)(param_3 + 0x30)) &
                *(ulonglong *)(lStack_50 + 0x30);
        lVar3 = *(longlong *)(lStack_50 + 8);
        lVar9 = *(longlong *)(*(longlong *)(lStack_50 + 0x18) + 8 + uVar8 * 0x10);
        if (lVar9 != lVar3) {
          uVar4 = *(ulonglong *)(lVar9 + 0x10);
          while( true ) {
            if ((*(ulonglong *)(param_3 + 0x30) == uVar4) &&
               (*(longlong *)(param_3 + 0x38) == *(longlong *)(lVar9 + 0x18))) goto LAB_14134a6d4;
            if (lVar9 == *(longlong *)(*(longlong *)(lStack_50 + 0x18) + uVar8 * 0x10)) break;
            lVar9 = *(longlong *)(lVar9 + 8);
            uVar4 = *(ulonglong *)(lVar9 + 0x10);
          }
        }
        lVar9 = 0;
LAB_14134a6d4:
        if (lVar9 == 0) {
          lVar9 = lVar3;
        }
        pcStack_60 = (char *)0x0;
        lStack_58 = 0;
        if (lVar5 != 0) {
          LOCK();
          piVar2 = (int *)(lVar5 + 8);
          *piVar2 = *piVar2 + -1;
          UNLOCK();
          if (*piVar2 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar5);
            LOCK();
            piVar2 = (int *)(lVar5 + 0xc);
            *piVar2 = *piVar2 + -1;
            UNLOCK();
            if (*piVar2 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar5);
            }
          }
          lVar5 = lStack_58;
          lStack_50 = 0;
          if (lStack_58 != 0) {
            LOCK();
            piVar2 = (int *)(lStack_58 + 8);
            *piVar2 = *piVar2 + -1;
            UNLOCK();
            if (*piVar2 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lStack_58);
              LOCK();
              piVar2 = (int *)(lVar5 + 0xc);
              *piVar2 = *piVar2 + -1;
              UNLOCK();
              if (*piVar2 == 0) {
                (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar5);
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
          *(undefined8 *)(lVar9 + 0xa0) = *puVar1;
          lVar3 = *(longlong *)(lVar9 + 0xa8);
          *(undefined8 *)(lVar9 + 0xa8) = uVar7;
          if (lVar3 != 0) {
            LOCK();
            piVar2 = (int *)(lVar3 + 8);
            *piVar2 = *piVar2 + -1;
            UNLOCK();
            if (*piVar2 == 0) {
              (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar3);
              LOCK();
              piVar2 = (int *)(lVar3 + 0xc);
              *piVar2 = *piVar2 + -1;
              UNLOCK();
              if (*piVar2 == 0) {
                (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)(lVar3);
              }
            }
          }
          (*(code *)PTR__guard_dispatch_icall_14f0fbaa8)
                    (*(undefined8 *)(param_1 + 0x3a0),param_3 + 0x30,puVar1);
        }
      }
    }
    *(undefined1 *)(param_3 + 0x11) = 1;
  }
  return;
}

