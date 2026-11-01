
longlong FUN_14298f4e0(longlong *param_1)

{
  code *pcVar1;
  longlong lVar2;
  ulonglong uVar3;
  longlong lVar4;
  longlong lVar5;
  longlong alStack_38 [2];
  longlong lStack_28;
  ulonglong uStack_20;
  
  lVar4 = *param_1;
  lVar5 = *(longlong *)(lVar4 + 0xd8) + *(longlong *)(lVar4 + 0xa8);
  for (lVar2 = *(longlong *)(lVar4 + 0xe0); lVar2 != *(longlong *)(lVar4 + 0xe8);
      lVar2 = lVar2 + 0x40) {
    lVar5 = lVar5 + *(longlong *)(lVar2 + 0x30);
  }
  FUN_14e932900(lVar4 + 0xf8,alStack_38);
  lVar4 = *(longlong *)(*param_1 + 0x148);
  if (0xf < uStack_20) {
    uVar3 = uStack_20 + 1;
    lVar2 = alStack_38[0];
    if (0xfff < uVar3) {
      lVar2 = *(longlong *)(alStack_38[0] + -8);
      if (0x1f < (ulonglong)((alStack_38[0] + -8) - lVar2)) {
        _invoke_watson(0,0,0,0,0);
        pcVar1 = (code *)swi(3);
        lVar4 = (*pcVar1)();
        return lVar4;
      }
      uVar3 = uStack_20 + 0x28;
    }
    FUN_14e934b00(lVar2,uVar3);
  }
  return lStack_28 + lVar5 + lVar4;
}

