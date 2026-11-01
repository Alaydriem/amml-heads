
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */

void FUN_14598a9d0(longlong param_1,longlong param_2,longlong *param_3,undefined8 param_4,
                  undefined8 param_5,float param_6,float param_7,float param_8,float param_9,
                  undefined8 param_10,float param_11,float param_12,float param_13,uint param_14,
                  uint param_15,undefined8 param_16,float param_17)

{
  int *piVar1;
  char cVar2;
  char *pcVar3;
  undefined8 uVar4;
  longlong lVar5;
  bool bVar6;
  undefined1 auVar7 [16];
  longlong lVar8;
  uint uVar9;
  uint uVar10;
  uint uVar11;
  uint uVar12;
  uint uVar13;
  int iVar14;
  int iVar15;
  undefined1 auVar16 [16];
  ulonglong uVar17;
  float fVar18;
  undefined4 in_XMM4_Db;
  undefined1 in_XMM5 [16];
  undefined1 auVar23 [12];
  undefined1 auVar19 [16];
  undefined1 auVar21 [16];
  undefined1 auVar22 [16];
  float fVar24;
  uint uVar25;
  longlong alStack_130 [2];
  longlong lStack_120;
  undefined1 uStack_118;
  float fStack_10c;
  undefined1 auStack_108 [16];
  longlong lStack_f8;
  ulonglong uStack_f0;
  undefined1 uStack_e8;
  undefined8 uStack_e0;
  undefined1 auVar20 [16];
  
  auVar23 = in_XMM5._4_12_;
  uStack_e0 = 0xfffffffffffffffe;
  pcVar3 = *(char **)(*(longlong *)(param_2 + 0x28) + 0xb0);
  if ((pcVar3[0x2a0] == '\0') && (pcVar3[0x255] == '\0')) {
    pcVar3[0x298] = '\0';
    pcVar3[0x299] = '\0';
    pcVar3[0x29a] = '\0';
    pcVar3[0x29b] = '\0';
    pcVar3[0x255] = '\0';
    pcVar3[0x2a0] = '\0';
    pcVar3[0x1ca] = '\0';
    func_0x000144df0270(pcVar3 + 8);
    *pcVar3 = '\0';
    pcVar3[0x1b4] = '\0';
    pcVar3[0x188] = '\0';
    pcVar3[0x1ba] = '\0';
    pcVar3[0x168] = '\0';
    pcVar3[0x194] = '\0';
    pcVar3[0x1a0] = '\0';
    pcVar3[0x1ac] = '\0';
    pcVar3[0x1be] = '\0';
    pcVar3[0x1c9] = '\0';
    pcVar3[8] = '\x01';
    pcVar3[0x254] = '\0';
    pcVar3[0x2a0] = '\x01';
    pcVar3[0x2a1] = '\0';
    pcVar3[0x20c] = '\0';
    pcVar3[0x20d] = '\0';
    pcVar3[0x20e] = '\0';
    pcVar3[0x20f] = '\0';
    if (*(longlong *)(pcVar3 + 0x1e8) != *(longlong *)(pcVar3 + 0x1f0)) {
      *(longlong *)(pcVar3 + 0x1f0) = *(longlong *)(pcVar3 + 0x1e8);
    }
    pcVar3[0x158] = '\x01';
    pcVar3[0x170] = '\x04';
    pcVar3[0x171] = '\0';
    pcVar3[0x172] = '\0';
    pcVar3[0x173] = '\0';
    pcVar3[0x174] = '\0';
    pcVar3[0x175] = '\0';
    pcVar3[0x176] = '\0';
    pcVar3[0x177] = '\0';
  }
  if (param_14 == 0xffffffff) {
    if (pcVar3[0x254] == '\0') {
      uVar9 = (uint)(param_12 * _DAT_14ef3fd58);
      if ((int)uVar9 < 1) {
        uVar9 = 0;
      }
      if (0xfe < (int)uVar9) {
        uVar9 = 0xff;
      }
      iVar14 = 0;
      if (0 < (int)(param_13 * _DAT_14ef3fd58)) {
        iVar14 = (int)(param_13 * _DAT_14ef3fd58);
      }
      if (0xfe < iVar14) {
        iVar14 = 0xff;
      }
      if (pcVar3[0x1b4] == '\0') {
        pcVar3[0x1b4] = '\x01';
      }
      *(uint *)(pcVar3 + 0x1b0) = uVar9 << 8 | uVar9 | uVar9 << 0x10 | iVar14 << 0x18;
      cVar2 = *pcVar3;
      goto joined_r0x00014598ad08;
    }
  }
  else if (pcVar3[0x254] == '\0') {
    auVar21._0_4_ = (float)(param_14 >> 8 & 0xff);
    auVar21._4_12_ = auVar23;
    auVar20._0_8_ = auVar21._0_8_;
    auVar20._8_4_ = auVar23._0_4_;
    auVar20._12_4_ = in_XMM4_Db;
    auVar19._8_8_ = auVar20._8_8_;
    auVar19._4_4_ = (float)(param_14 & 0xff);
    auVar19._0_4_ = auVar21._0_4_;
    auVar21 = divps(auVar19,_DAT_14f0901e0);
    auVar22._4_4_ = param_12 * auVar21._4_4_;
    auVar22._0_4_ = param_12 * auVar21._0_4_;
    auVar22._8_4_ = param_12 * auVar21._8_4_;
    auVar22._12_4_ = param_12 * auVar21._12_4_;
    auVar22 = maxps(ZEXT816(0),auVar22);
    iVar14 = -(uint)(_DAT_14efd2400 < auVar22._0_4_);
    iVar15 = -(uint)(_UNK_14efd2404 < auVar22._4_4_);
    param_12 = ((float)(param_14 >> 0x10 & 0xff) / _DAT_14ef3fd58) * param_12;
    fVar18 = 0.0;
    if (0.0 <= param_12) {
      fVar18 = param_12;
    }
    fVar24 = _DAT_14eec7060;
    if (fVar18 <= _DAT_14eec7060) {
      fVar24 = fVar18;
    }
    uVar9 = 0;
    uVar25 = (int)(fVar24 * _DAT_14ef3fd58);
    if ((int)(fVar24 * _DAT_14ef3fd58) < 1) {
      uVar25 = uVar9;
    }
    if (0xfe < (int)uVar25) {
      uVar25 = 0xff;
    }
    uVar10 = (uint)(auVar22._0_4_ * _DAT_14ef3fd58);
    if ((int)uVar10 < 1) {
      uVar10 = uVar9;
    }
    if (0xfe < (int)uVar10) {
      uVar10 = 0xff;
    }
    uVar11 = (uint)(auVar22._4_4_ * _DAT_14ef3fd58);
    if ((int)uVar11 < 1) {
      uVar11 = uVar9;
    }
    if (0xfe < (int)uVar11) {
      uVar11 = 0xff;
    }
    uVar12 = (uint)(param_13 * _DAT_14ef3fd58);
    if (0 < (int)uVar12) {
      uVar9 = uVar12;
    }
    auVar7._4_4_ = iVar14;
    auVar7._0_4_ = iVar14;
    auVar7._8_4_ = iVar15;
    auVar7._12_4_ = iVar15;
    uVar12 = movmskpd(uVar12,auVar7);
    if (0xfe < (int)uVar9) {
      uVar9 = 0xff;
    }
    uVar13 = 0xff0000;
    if ((uVar12 & 2) == 0) {
      uVar13 = uVar11 << 0x10;
    }
    uVar11 = 0xff00;
    if ((uVar12 & 1) == 0) {
      uVar11 = uVar10 << 8;
    }
    if (pcVar3[0x1b4] == '\0') {
      pcVar3[0x1b4] = '\x01';
    }
    *(uint *)(pcVar3 + 0x1b0) = uVar11 | uVar9 << 0x18 | uVar13 | uVar25;
    cVar2 = *pcVar3;
joined_r0x00014598ad08:
    if (cVar2 == '\0') {
      pcVar3[0x159] = '\x01';
    }
  }
  fVar18 = _DAT_14eec7060;
  fStack_10c = (param_17 + _DAT_14ef1c470) * _DAT_14eee6c00 * param_11;
  bVar6 = _DAT_14eec7060 < param_6;
  if (param_6 <= 0.0) {
    param_6 = 0.0;
  }
  uVar9 = -(uint)bVar6 & (uint)_DAT_14eec7060 | ~-(uint)bVar6 & (uint)param_6;
  bVar6 = _DAT_14eec7060 < param_9;
  if (param_9 <= 0.0) {
    param_9 = 0.0;
  }
  uVar25 = -(uint)bVar6 & (uint)_DAT_14eec7060 | ~-(uint)bVar6 & (uint)param_9;
  *(uint *)(pcVar3 + 0x18c) = uVar9;
  if (pcVar3[0x194] == '\0') {
    pcVar3[0x194] = '\x01';
  }
  *(uint *)(pcVar3 + 400) = uVar25;
  if (*pcVar3 == '\0') {
    pcVar3[0x15c] = '\x01';
  }
  FUN_1454175e0(pcVar3);
  bVar6 = fVar18 < param_7;
  if (param_7 <= 0.0) {
    param_7 = 0.0;
  }
  uVar10 = -(uint)bVar6 & (uint)fVar18 | ~-(uint)bVar6 & (uint)param_7;
  *(uint *)(pcVar3 + 0x18c) = uVar10;
  if (pcVar3[0x194] == '\0') {
    pcVar3[0x194] = '\x01';
  }
  *(uint *)(pcVar3 + 400) = uVar25;
  if (*pcVar3 == '\0') {
    pcVar3[0x15c] = '\x01';
  }
  FUN_1454175e0(pcVar3);
  bVar6 = fVar18 < param_8;
  if (param_8 <= 0.0) {
    param_8 = 0.0;
  }
  uVar25 = -(uint)bVar6 & (uint)fVar18 | ~-(uint)bVar6 & (uint)param_8;
  *(uint *)(pcVar3 + 0x18c) = uVar10;
  if (pcVar3[0x194] == '\0') {
    pcVar3[0x194] = '\x01';
  }
  *(uint *)(pcVar3 + 400) = uVar25;
  if (*pcVar3 == '\0') {
    pcVar3[0x15c] = '\x01';
  }
  FUN_1454175e0(pcVar3);
  *(uint *)(pcVar3 + 0x18c) = uVar9;
  if (pcVar3[0x194] == '\0') {
    pcVar3[0x194] = '\x01';
  }
  *(uint *)(pcVar3 + 400) = uVar25;
  if (*pcVar3 == '\0') {
    pcVar3[0x15c] = '\x01';
  }
  FUN_1454175e0(pcVar3);
  uVar4 = *(undefined8 *)(param_2 + 0x28);
  uVar17 = CONCAT44(param_15 >> 0x10,param_15 >> 8 & _DAT_14f08b730) & 0xffffffffff;
  auVar16._0_4_ = (float)(int)(uVar17 >> 0x20);
  auVar16._4_4_ = (float)(int)uVar17;
  auVar16._8_4_ = (float)(int)(param_15 & _UNK_14f08b734);
  auVar16._12_4_ = (float)(param_15 >> 0x18);
  auStack_108 = divps(auVar16,_DAT_14f0827e0);
  func_0x0001457cb9c0(uVar4,&UNK_14f0feaf0,auStack_108,&UNK_14f0feaf0);
  lVar8 = *param_3;
  if (lVar8 == 0) {
    uStack_f0 = uStack_f0 & 0xffffffffffffff00;
    FUN_145ad5f90(uVar4,pcVar3,param_1 + 0x168,auStack_108);
    switch(uStack_f0 & 0xff) {
    default:
      goto LAB_14598b17b;
    case 1:
      lVar8 = auStack_108._0_8_;
      break;
    case 2:
    case 3:
      lVar8 = lStack_f8;
      goto joined_r0x00014598afe1;
    }
  }
  else {
    uStack_118 = 0;
    if (param_3[1] == 0) {
      auStack_108._8_8_ = 0;
    }
    else {
      LOCK();
      piVar1 = (int *)(param_3[1] + 8);
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      lVar8 = *param_3;
      auStack_108._8_8_ = param_3[1];
    }
    auStack_108._0_8_ = lVar8;
    if (param_3[3] == 0) {
      uStack_f0 = 0;
    }
    else {
      LOCK();
      piVar1 = (int *)(param_3[3] + 8);
      *piVar1 = *piVar1 + 1;
      UNLOCK();
      uStack_f0 = param_3[3];
    }
    lStack_f8 = param_3[2];
    uStack_e8 = 1;
    FUN_145ad65e0(uVar4,pcVar3,param_1 + 0x168,auStack_108,alStack_130);
    uVar17 = uStack_f0;
    switch(uStack_e8) {
    case 1:
      if (uStack_f0 != 0) {
        LOCK();
        piVar1 = (int *)(uStack_f0 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uStack_f0);
          LOCK();
          piVar1 = (int *)(uVar17 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uVar17);
          }
        }
      }
    case 2:
    case 3:
      uVar4 = auStack_108._8_8_;
      if (auStack_108._8_8_ != 0) {
        LOCK();
        piVar1 = (int *)(auStack_108._8_8_ + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(auStack_108._8_8_);
          LOCK();
          piVar1 = (int *)(uVar4 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(uVar4);
          }
        }
      }
    }
    switch(uStack_118) {
    default:
      goto LAB_14598b17b;
    case 1:
      lVar8 = alStack_130[0];
      break;
    case 2:
    case 3:
      lVar8 = lStack_120;
joined_r0x00014598afe1:
      if (lVar8 != 0) {
        LOCK();
        piVar1 = (int *)(lVar8 + 8);
        *piVar1 = *piVar1 + -1;
        UNLOCK();
        if (*piVar1 == 0) {
          (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar8);
          LOCK();
          piVar1 = (int *)(lVar8 + 0xc);
          *piVar1 = *piVar1 + -1;
          UNLOCK();
          if (*piVar1 == 0) {
            (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar8);
          }
        }
      }
      goto LAB_14598b17b;
    }
  }
  if (lVar8 != 0) {
    lVar5 = *(longlong *)(lVar8 + 8);
    if (lVar5 != 0) {
      LOCK();
      piVar1 = (int *)(lVar5 + 0xc);
      *piVar1 = *piVar1 + -1;
      UNLOCK();
      if (*piVar1 == 0) {
        (*(code *)PTR__guard_dispatch_icall_14f99ef70)(lVar5);
      }
    }
    FUN_14e934b00(lVar8,0x20);
  }
LAB_14598b17b:
  return;
}

