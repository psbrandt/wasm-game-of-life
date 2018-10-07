/* tslint:disable */
export enum Cell {Dead,Alive,}
export class Universe {
free(): void;
 tick(): void;

static  new(): Universe;

 render(): string;

 width(): number;

 height(): number;

 cells(): number;

 toggle_cell(arg0: number, arg1: number): void;

}
