import { expect, it } from 'vitest';
import { plus100 } from '../napi';

it(() => expect(plus100(1)).toBe(101));
