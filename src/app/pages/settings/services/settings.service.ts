import { Injectable } from '@angular/core';
import { MatSnackBar } from '@angular/material/snack-bar';
import { invoke } from '@tauri-apps/api/tauri';
import { BehaviorSubject } from 'rxjs';

const CMD_GET_PRINTERS = 'get_printers';
const CMD_SET_PRINTER = 'set_printer';
const CMD_GET_SETTINGS = 'get_settings';
const CMD_PRINT_TEST_PAGE = 'print_test_page';

interface Printer {
  id: string;
  name: string;
}

interface Settings {
  printer_id?: string;
}

@Injectable()
export class SettingsService {
  private printersSubject = new BehaviorSubject<Printer[]>([]);
  public printers$ = this.printersSubject.asObservable();

  private settingsSubject = new BehaviorSubject<Settings>({});
  public settings$ = this.settingsSubject.asObservable();

  constructor(private snackbar: MatSnackBar) {
    this.getPrinters();
    this.getSettings();
  }

  public async getPrinters() {
    const printers = await invoke<Printer[]>(CMD_GET_PRINTERS);
    this.printersSubject.next(printers);
  }

  public async getSettings() {
    const settings = await invoke<Settings>(CMD_GET_SETTINGS);
    this.settingsSubject.next(settings);
  }

  public async setPrinter(printerId: string) {
    try {
      await invoke(CMD_SET_PRINTER, { printerId });
      this.snackbar.open('✅ Drucker festgelegt!', undefined, { duration: 3000 });
    } catch (error) {
      this.snackbar.open(`❌ Das hat nicht geklappt: ${(error as Error).toString()}`, undefined, { duration: 3000 });
    }
  }

  public async printTestPage() {
    try {
      await invoke(CMD_PRINT_TEST_PAGE);
      this.snackbar.open('✅ Test-Seite wird gedruckt!', undefined, { duration: 3000 });
    } catch (error) {
      this.snackbar.open(`❌ Das hat nicht geklappt: ${(error as Error).toString()}`, undefined, { duration: 3000 });
    }
  }
}
