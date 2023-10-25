import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-layout-section',
  templateUrl: './layout-section.component.html',
  styleUrls: ['./layout-section.component.scss'],
})
export class LayoutSectionComponent {
  @Input() public fullWidth = false;
}
