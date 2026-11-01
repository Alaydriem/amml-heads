
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

ulonglong FUN_145421340(longlong param_1,longlong param_2,longlong param_3)

{
  longlong *plVar1;
  longlong lVar2;
  undefined8 uVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong lVar6;
  longlong unaff_GS_OFFSET;
  undefined1 auStack_c8 [48];
  undefined1 auStack_98 [32];
  longlong lStack_78;
  undefined8 uStack_70;
  undefined8 uStack_68;
  longlong lStack_60;
  undefined8 uStack_58;
  longlong lStack_50;
  longlong lStack_48;
  undefined1 uStack_39;
  undefined8 uStack_38;
  
  uStack_38 = 0xfffffffffffffffe;
  lStack_48 = param_3;
  lVar4 = FUN_140ab6b70((longlong *)(param_1 + 0x1a8));
  lVar6 = lStack_48;
  if (lVar4 == *(longlong *)(param_1 + 0x1a8)) {
    uVar5 = 0;
  }
  else {
    plVar1 = *(longlong **)(lVar4 + 0x58);
    if (((((int)plVar1[2] == *(int *)(lStack_48 + 0x18)) &&
         (*(int *)((longlong)plVar1 + 0x14) == *(int *)(lStack_48 + 0x1c))) &&
        ((int)plVar1[3] == *(int *)(lStack_48 + 0x20))) && (lVar2 = *plVar1, lVar2 != 0)) {
      if ((*(int *)(*(longlong *)
                     (*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8) + 4) <
           _DAT_1528d6fc8) && (FUN_14e9380b0(&DAT_1528d6fc8), _DAT_1528d6fc8 == -1)) {
        FUN_14e934b40(FUN_140a771c0);
        FUN_14e938170(&DAT_1528d6fc8);
      }
      LOCK();
      *(short *)(lVar2 + 0x10) = *(short *)(lVar2 + 0x10) + 1;
      UNLOCK();
      if ((*(byte *)(lVar2 + 0x12) & 1) != 0) {
        FUN_144c73fb0(&uStack_58,*(undefined8 *)(lVar4 + 0x58));
        uVar3 = *(undefined8 *)(param_1 + 0x1d0);
        FUN_145a4bd30(auStack_c8,lVar6);
        uStack_70 = *(undefined8 *)(param_2 + 0x18);
        if (*(ulonglong *)(param_2 + 0x20) < 0x10) {
          param_2 = param_2 + 8;
        }
        else {
          param_2 = *(longlong *)(param_2 + 8);
        }
        uStack_39 = 1;
        lStack_78 = param_2;
        FUN_1400ba710(auStack_98,&lStack_78);
        if (lStack_50 == 0) {
          lStack_50 = 0;
        }
        else {
          LOCK();
          *(int *)(lStack_50 + 8) = *(int *)(lStack_50 + 8) + 1;
          UNLOCK();
        }
        uStack_68 = uStack_58;
        lStack_60 = lStack_50;
        FUN_1454215f0(uVar3,&uStack_68,auStack_c8);
        FUN_145421680(auStack_c8);
        FUN_14007e6d0(&uStack_58);
        uVar5 = CONCAT71((int7)((ulonglong)param_2 >> 8),1);
        lVar6 = lStack_48;
        goto LAB_145421423;
      }
    }
    uVar5 = 0;
  }
LAB_145421423:
  if (*(longlong *)(lVar6 + 8) != 0) {
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
  }
  return uVar5 & 0xffffffff;
}

