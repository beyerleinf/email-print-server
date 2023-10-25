import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { LayoutComponent } from './components/layout/layout.component';
import { LayoutContentDirective, LayoutFooterDirective, LayoutHeaderDirective } from './directives/layout.directives';
import { LayoutSectionComponent } from './components/layout-section/layout-section.component';

@NgModule({
  declarations: [
    LayoutComponent,
    LayoutContentDirective,
    LayoutFooterDirective,
    LayoutHeaderDirective,
    LayoutSectionComponent,
  ],
  imports: [CommonModule],
  exports: [
    LayoutComponent,
    LayoutContentDirective,
    LayoutFooterDirective,
    LayoutHeaderDirective,
    LayoutSectionComponent,
  ],
})
export class SharedCommonModule {}
