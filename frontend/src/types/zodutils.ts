import * as zod from "zod";

export type ZodValues<T extends zod.ZodRawShape> = zod.infer<zod.ZodObject<T>>;
export type ZodKey<T extends zod.ZodRawShape> = keyof ZodValues<T>;
export type ZodShape<T extends zod.ZodObject<zod.ZodRawShape>> = T extends zod.ZodObject<infer V & zod.ZodRawShape> ? V : never;