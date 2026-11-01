
/* WARNING: Removing unreachable block (ram,0x000140e9b1f3) */
/* WARNING: Removing unreachable block (ram,0x000140e9b1fa) */
/* WARNING: Removing unreachable block (ram,0x000140e9b210) */
/* WARNING: Removing unreachable block (ram,0x000140e9b220) */
/* WARNING: Removing unreachable block (ram,0x000140e9b2c6) */
/* WARNING: Removing unreachable block (ram,0x000140e9b2cd) */
/* WARNING: Removing unreachable block (ram,0x000140e9b2e3) */
/* WARNING: Type propagation algorithm not settling */
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_140e9b010(longlong param_1,undefined8 **param_2,longlong *param_3,ulonglong *param_4,
                  undefined4 param_5,undefined4 *param_6)

{
  int *piVar1;
  code *pcVar2;
  char cVar3;
  undefined8 *puVar4;
  longlong lVar5;
  longlong lVar6;
  undefined8 ****ppppuVar7;
  ulonglong uVar8;
  ulonglong *puVar9;
  ulonglong *puVar10;
  longlong unaff_GS_OFFSET;
  undefined4 uVar11;
  undefined4 uVar12;
  ulonglong in_stack_fffffffffffffe68;
  undefined4 uStack_178;
  undefined4 uStack_174;
  undefined4 uStack_170;
  undefined4 uStack_16c;
  undefined *puStack_160;
  undefined8 uStack_158;
  char **ppcStack_150;
  longlong lStack_148;
  longlong lStack_140;
  ulonglong uStack_138;
  ulonglong uStack_130;
  undefined4 uStack_128;
  longlong *plStack_120;
  longlong lStack_118;
  char cStack_110;
  undefined7 uStack_10f;
  undefined1 auStack_108 [8];
  undefined8 uStack_100;
  char ****ppppcStack_f8;
  longlong lStack_f0;
  longlong lStack_e8;
  undefined1 uStack_c0;
  undefined8 ****ppppuStack_b8;
  ulonglong *puStack_b0;
  undefined8 ****ppppuStack_a8;
  longlong lStack_a0;
  ulonglong uStack_98;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  ushort uStack_70;
  undefined8 uStack_60;
  
  uStack_60 = 0xfffffffffffffffe;
  func_0x00014e942578(0x1528c2fd0);
  LOCK();
  _DAT_1528c3030 = _DAT_1528c3030 + 1;
  UNLOCK();
  func_0x00014e94259c(0x1528c2fd0);
  if ((_DAT_1528c3038 == (undefined8 ****)0x0) || (*(char *)_DAT_1528c3038 != '\x01')) {
    ppppuStack_b8 = (undefined8 ****)0x0;
    puStack_b0 = (ulonglong *)((ulonglong)puStack_b0._1_7_ << 8);
    ppppuStack_a8 = (undefined8 ****)0x0;
    lStack_a0 = 0;
    uStack_98 = 0;
    FUN_1403b2020(0x1528c2fd0);
LAB_140e9b0d3:
    lStack_e8 = 0;
LAB_140e9b0db:
    uVar8 = 0;
  }
  else {
    if (_DAT_1528c3040 == 0) {
      lStack_a0 = 0;
    }
    else {
      LOCK();
      *(int *)(_DAT_1528c3040 + 8) = *(int *)(_DAT_1528c3040 + 8) + 1;
      UNLOCK();
      lStack_a0 = _DAT_1528c3040;
    }
    lStack_e8 = _DAT_1528c3048;
    ppppcStack_f8 = (char ****)_DAT_1528c3038;
    uStack_98 = _DAT_1528c3048;
    ppppuStack_b8 = (undefined8 ****)0x1528c2fd0;
    puStack_b0 = (ulonglong *)CONCAT71(puStack_b0._1_7_,1);
    ppppuStack_a8 = _DAT_1528c3038;
    if (lStack_a0 == 0) {
      lVar6 = 0;
    }
    else {
      LOCK();
      *(int *)(lStack_a0 + 8) = *(int *)(lStack_a0 + 8) + 1;
      UNLOCK();
      lVar6 = lStack_a0;
    }
    if ((ppppcStack_f8 == (char ****)0x0) || (*(char *)ppppcStack_f8 != '\x01')) {
      ppppcStack_f8 = (char ****)0x0;
      lStack_f0 = 0;
      if (lVar6 == 0) goto LAB_140e9b0d3;
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
      lVar6 = lStack_f0;
      lStack_e8 = 0;
      if (lStack_f0 != 0) {
        LOCK();
        piVar1 = (int *)(lStack_f0 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_f0);
          LOCK();
          piVar1 = (int *)(lVar6 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar6);
          }
        }
      }
      goto LAB_140e9b0db;
    }
    lStack_f0 = lVar6;
    cVar3 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)();
    ppppcStack_f8 = (char ****)0x0;
    lStack_f0 = 0;
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
      lVar6 = lStack_f0;
      lStack_e8 = 0;
      if (lStack_f0 != 0) {
        LOCK();
        piVar1 = (int *)(lStack_f0 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_f0);
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
    uVar8 = uStack_98;
    if (cVar3 == '\0') goto LAB_140e9b0db;
    if (lStack_a0 == 0) {
      lVar6 = 0;
    }
    else {
      LOCK();
      *(int *)(lStack_a0 + 8) = *(int *)(lStack_a0 + 8) + 1;
      UNLOCK();
      lVar6 = lStack_a0;
    }
    if ((ppppuStack_a8 != (undefined8 ****)0x0) && (*(char *)ppppuStack_a8 == '\0')) {
      _DAT_00000000 = 0xdeadc0de;
    }
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
    }
  }
  lVar6 = lStack_a0;
  ppppuStack_a8 = (undefined8 ****)0x0;
  lStack_a0 = 0;
  if (lVar6 == 0) {
    uStack_98 = 0;
  }
  else {
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
    lVar6 = lStack_a0;
    uStack_98 = 0;
    if (lStack_a0 != 0) {
      LOCK();
      piVar1 = (int *)(lStack_a0 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_a0);
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
  if ((char)puStack_b0 == '\x01') {
    FUN_1403b2020(ppppuStack_b8);
  }
  if (uVar8 != 0) {
    puVar4 = (undefined8 *)0x1529bf210;
    if ((undefined8 *)*param_3 != (undefined8 *)0x0) {
      puVar4 = (undefined8 *)*param_3;
    }
    if (puVar4[1] == 0) {
      ppppuStack_b8 = (undefined8 ****)*puVar4;
      puStack_b0 = (ulonglong *)0x0;
    }
    else {
      LOCK();
      piVar1 = (int *)(puVar4[1] + 8);
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      ppppuStack_b8 = (undefined8 ****)*puVar4;
      puStack_b0 = (ulonglong *)puVar4[1];
    }
    uStack_98 = CONCAT71(uStack_98._1_7_,2);
    if (puStack_b0 == (ulonglong *)0x0) {
      lStack_f0 = 0;
    }
    else {
      LOCK();
      *(int *)((longlong)puStack_b0 + 8) = *(int *)((longlong)puStack_b0 + 8) + 1;
      UNLOCK();
      lStack_f0 = (longlong)puStack_b0;
    }
    ppppcStack_f8 = (char ****)ppppuStack_b8;
    uStack_128 = 0;
    uStack_130 = 0;
    uStack_138 = 0;
    lStack_140 = 0;
    lStack_148 = 0;
    ppcStack_150 = (char **)param_2;
    FUN_144dfb3a0(&lStack_118,&ppppcStack_f8);
    lVar6 = lStack_f0;
    if (lStack_f0 != 0) {
      LOCK();
      piVar1 = (int *)(lStack_f0 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_f0);
        LOCK();
        piVar1 = (int *)(lVar6 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar6);
        }
      }
    }
    lVar6 = lStack_a0;
    switch(uStack_98 & 0xff) {
    case 1:
      if (lStack_a0 != 0) {
        LOCK();
        piVar1 = (int *)(lStack_a0 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_a0);
          LOCK();
          piVar1 = (int *)(lVar6 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar6);
          }
        }
      }
    case 2:
    case 3:
      puVar9 = puStack_b0;
      if (puStack_b0 != (ulonglong *)0x0) {
        LOCK();
        piVar1 = (int *)((longlong)puStack_b0 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(puStack_b0);
          LOCK();
          piVar1 = (int *)((longlong)puVar9 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(puVar9);
          }
        }
      }
    }
    lVar6 = lStack_140;
    if (CONCAT71(uStack_10f,cStack_110) == 0) {
      lVar5 = 0;
    }
    else {
      LOCK();
      piVar1 = (int *)(CONCAT71(uStack_10f,cStack_110) + 8);
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      lVar5 = CONCAT71(uStack_10f,cStack_110);
    }
    lStack_148 = lStack_118;
    if (lStack_140 != 0) {
      LOCK();
      piVar1 = (int *)(lStack_140 + 8);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        lStack_140 = lVar5;
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar6);
        LOCK();
        piVar1 = (int *)(lVar6 + 0xc);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        lVar5 = lStack_140;
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar6);
          lVar5 = lStack_140;
        }
      }
    }
    lStack_140 = lVar5;
    lVar6 = CONCAT71(uStack_10f,cStack_110);
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
    }
    uStack_138 = *param_4;
    uStack_130 = param_4[1];
    uStack_128 = param_5;
    ppppuStack_b8 = (undefined8 ****)&ppcStack_150;
    lVar6 = *(longlong *)(param_1 + 0x18);
    *(undefined1 *)(lVar6 + 0x78) = 1;
    puVar4 = *(undefined8 **)
              (*(longlong *)(lVar6 + 0x48) +
              (*(longlong *)(lVar6 + 0x50) - 1U &
              (*(longlong *)(lVar6 + 0x60) + *(longlong *)(lVar6 + 0x58)) - 1U) * 8);
    puStack_b0 = (ulonglong *)*puVar4;
    ppppuStack_a8 = (undefined8 ****)puVar4[1];
    lStack_a0 = puVar4[2];
    uStack_98 = puVar4[3];
    uStack_90 = *(undefined4 *)(puVar4 + 4);
    uStack_8c = *(undefined4 *)((longlong)puVar4 + 0x24);
    uStack_88 = *(undefined4 *)(puVar4 + 5);
    uStack_84 = *(undefined4 *)((longlong)puVar4 + 0x2c);
    uStack_80 = *(undefined4 *)(puVar4 + 6);
    uStack_7c = *(undefined4 *)((longlong)puVar4 + 0x34);
    uStack_78 = *(undefined4 *)(puVar4 + 7);
    uStack_74 = *(undefined4 *)((longlong)puVar4 + 0x3c);
    uStack_70 = (ushort)*(byte *)(param_1 + 0x74);
    if ((*(int *)(*(longlong *)(*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8)
                 + 4) < _DAT_1528e0dd8) && (FUN_14e9380b0(&DAT_1528e0dd8), _DAT_1528e0dd8 == -1)) {
      puStack_160 = &UNK_14f52d07a;
      uStack_158 = 0x15;
      uStack_178 = _DAT_14f0bbc18;
      uStack_174 = _UNK_14f0bbc1c;
      uStack_170 = _UNK_14f0bbc20;
      uStack_16c = _UNK_14f0bbc24;
      FUN_1402fe330(0x1528e0ca0,&uStack_178,1,&puStack_160,
                    in_stack_fffffffffffffe68 & 0xffffffff00000000,&UNK_14f0bbae8);
      FUN_14e934b40(FUN_140ec9c70);
      FUN_14e938170(&DAT_1528e0dd8);
    }
    lStack_118 = 0x1528e0ca0;
    cStack_110 = 0;
    uStack_100 = 0;
    uStack_c0 = 0;
    cStack_110 = FUN_1439a7de0(0x1528e0ca0,auStack_108,&ppppcStack_f8);
    ppppcStack_f8 = (char ****)&ppppuStack_b8;
    lStack_f0 = CONCAT71(lStack_f0._1_7_,1);
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uVar8,&ppppcStack_f8);
    if (cStack_110 == '\x01') {
      FUN_1477a1900(&DAT_15244dd00,auStack_108,lStack_118 + 0x60);
    }
    lVar6 = lStack_140;
    if (lStack_140 == 0) {
      return;
    }
    LOCK();
    piVar1 = (int *)(lStack_140 + 8);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 != 0) {
      return;
    }
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lStack_140);
    LOCK();
    piVar1 = (int *)(lVar6 + 0xc);
    *piVar1 = *piVar1 + -1;
    UNLOCK();
    if (*piVar1 != 0) {
      return;
    }
    (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar6);
    return;
  }
  ppppuVar7 = (undefined8 ****)*param_4;
  if (ppppuVar7 == (undefined8 ****)0x0) {
    return;
  }
  plStack_120 = param_3;
  if (_DAT_15294c2f0 == 0) {
    lVar6 = (*(code *)PTR__guard_dispatch_icall_14f99ef70)(_DAT_15233c1a8,1);
    if (lVar6 == 0) {
      FUN_1409bc820(1);
    }
    ppppuVar7 = (undefined8 ****)*param_4;
    ppppuStack_b8 = (undefined8 ****)0x0;
    puStack_b0 = (ulonglong *)0x0;
    ppppuStack_a8 = (undefined8 ****)0x0;
    _DAT_15294c2f0 = lVar6;
    if (ppppuVar7 != (undefined8 ****)0x0) goto LAB_140e9b85a;
    lVar5 = 0;
    ppppcStack_f8 = (char ****)ppppuVar7;
  }
  else {
LAB_140e9b85a:
    lVar6 = _DAT_15294c2f0;
    ppppuStack_a8 = (undefined8 ****)0x0;
    puStack_b0 = (ulonglong *)0x0;
    ppppuStack_b8 = (undefined8 ****)0x0;
    ppppcStack_f8 = (char ****)ppppuVar7;
    if ((ulonglong)ppppuVar7 >> 0x3d != 0) {
      FUN_140075b60();
      goto LAB_140e9baa3;
    }
    FUN_1402fb1c0(&ppppuStack_b8,&ppppcStack_f8);
    lVar5 = *param_4 * 0xc;
  }
  puVar9 = (ulonglong *)param_4[1];
  puVar10 = (ulonglong *)(lVar5 + (longlong)puVar9);
  for (; puVar9 != puVar10; puVar9 = (ulonglong *)((longlong)puVar9 + 0xc)) {
    while (ppppcStack_f8 = (char ****)*puVar9, (undefined8 ****)puStack_b0 == ppppuStack_a8) {
      FUN_1400a4ec0(&ppppuStack_b8,puStack_b0,&ppppcStack_f8);
      puVar9 = (ulonglong *)((longlong)puVar9 + 0xc);
      if (puVar9 == puVar10) goto LAB_140e9b8f6;
    }
    *puStack_b0 = (ulonglong)ppppcStack_f8;
    puStack_b0 = puStack_b0 + 1;
  }
LAB_140e9b8f6:
  uVar11 = _DAT_14f0883d8;
  uVar12 = _DAT_14f0883d8;
  if (param_6 != (undefined4 *)0x0) {
    uVar11 = *param_6;
    uVar12 = param_6[1];
  }
  FUN_1424e6000(lVar6,&ppppuStack_b8,uVar11,uVar12,param_1,plStack_120,0);
  if (ppppuStack_b8 != (undefined8 ****)0x0) {
    uVar8 = (longlong)ppppuStack_a8 - (longlong)ppppuStack_b8;
    ppppuVar7 = ppppuStack_b8;
    if (0xfff < uVar8) {
      ppppuVar7 = (undefined8 ****)ppppuStack_b8[-1];
      if (0x1f < (ulonglong)((longlong)ppppuStack_b8 + (-8 - (longlong)ppppuVar7))) {
LAB_140e9baa3:
        _invoke_watson(0,0,0,0,0);
        pcVar2 = (code *)swi(3);
        (*pcVar2)();
        return;
      }
      uVar8 = uVar8 + 0x27;
    }
    FUN_14e934b00(ppppuVar7,uVar8);
  }
  return;
}

