import type {
  VNodeChild,
  ComponentPublicInstance,
  FunctionalComponent,
  PropType as VuePropType,
} from 'vue';

declare global {
  declare type PropType<T> = VuePropType<T>;
  declare type VueNode = VNodeChild | JSX.Element;

  declare type Fn = () => void;

  declare type Writable<T> = {
    -readonly [P in keyof T]: T[P];
  };

  declare type Indexable<T> = {
    [key: string]: T;
  };

  declare type Mutable<T> = { -readonly [P in keyof T]: T[P] };

  declare type Nullable<T> = T | null;
  declare type NonNullable<T> = T extends null | undefined ? never : T;
  declare type Recordable<T> = Record<string, T>;
  declare type ReadonlyRecordable<T> = {
    readonly [key: string]: T;
  };
  declare type Indexable<T> = {
    [key: string]: T;
  };

  declare type IntervalHandle = ReturnType<typeof setInterval>;

  import '@ant-design/icons-vue';
  import 'ant-design-vue/typings/global';

  declare interface ViteEnv {
    VITE_PORT: number;
  }
}

declare module 'vue' {
  export type JSXComponent<Props> =
    | { new (): ComponentPublicInstance<Props> }
    | FunctionalComponent<Props>;
}
