import { CommonModule } from '@angular/common';
import { NgModule } from '@angular/core';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatSnackBarModule } from '@angular/material/snack-bar';
import { RouterModule } from '@angular/router';
import { SharedCommonModule } from 'src/app/shared/common/shared-common.module';
import { ContentSectionComponent } from './sections/content-section/content-section.component';
import { SettingsComponent } from './settings.component';

@NgModule({
  declarations: [SettingsComponent, ContentSectionComponent],
  imports: [
    CommonModule,
    SharedCommonModule,
    MatSelectModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatSnackBarModule,
    RouterModule.forChild([{ path: '', component: SettingsComponent }]),
  ],
})
export default class SettingsModule {}
