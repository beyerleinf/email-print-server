import { Directive } from '@angular/core';

@Directive({
  selector: '[appLayoutHeader]',
})
export class LayoutHeaderDirective {}

@Directive({
  selector: '[appLayoutContent]',
})
export class LayoutContentDirective {}

@Directive({
  selector: '[appLayoutFooter]',
})
export class LayoutFooterDirective {}
