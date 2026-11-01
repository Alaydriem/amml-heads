
/* WARNING: Globals starting with '_' overlap smaller symbols at the same address */
/* WARNING: Restarted to delay deadcode elimination for space: ram */

ulonglong FUN_14541b7a0(longlong param_1,undefined8 param_2,char param_3,uint param_4)

{
  longlong lVar1;
  longlong lVar2;
  undefined8 *puVar3;
  longlong lVar4;
  ulonglong uVar5;
  longlong unaff_GS_OFFSET;
  bool bVar6;
  
  if (*(char **)(param_1 + 8) == (char *)0x0) {
    _DAT_00000000 = 0xdeadc0de;
  }
  else if (**(char **)(param_1 + 8) == '\0') {
    _DAT_00000000 = 0xdeadc0de;
  }
  lVar1 = *(longlong *)(param_1 + 0x18);
  lVar4 = lVar1 + 0x1a8;
  lVar2 = FUN_140ab6b70(lVar4);
  bVar6 = lVar2 != *(longlong *)(lVar1 + 0x1a8);
  uVar5 = CONCAT71((int7)((ulonglong)lVar4 >> 8),bVar6);
  if (param_3 == '\0' && bVar6) {
    if ((param_4 & 0xff) - 8 < 2) {
      puVar3 = (undefined8 *)(lVar2 + 0x78);
    }
    else if ((param_4 & 0xff) - 2 < 2) {
      puVar3 = (undefined8 *)(lVar2 + 0x68);
    }
    else {
      puVar3 = (undefined8 *)(lVar2 + 0x58);
    }
    lVar4 = *(longlong *)*puVar3;
    if (lVar4 == 0) {
      uVar5 = 0;
    }
    else {
      if ((*(int *)(*(longlong *)
                     (*(longlong *)(unaff_GS_OFFSET + 0x58) + (ulonglong)__tls_index * 8) + 4) <
           _DAT_1528d6fc8) && (FUN_14e9380b0(&DAT_1528d6fc8), _DAT_1528d6fc8 == -1)) {
        FUN_14e934b40(FUN_140a771c0);
        FUN_14e938170(&DAT_1528d6fc8);
      }
      LOCK();
      *(short *)(lVar4 + 0x10) = *(short *)(lVar4 + 0x10) + 1;
      UNLOCK();
      uVar5 = (ulonglong)*(byte *)(lVar4 + 0x12);
    }
  }
  return uVar5 & 0xffffff01;
}

