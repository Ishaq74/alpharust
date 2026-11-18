@@meta when=react&!official@@
import { useState } from 'react';

export default function Counter() {
  const [count, setCount] = useState(0);
  return (
    <button type="button" className="button" onClick={() => setCount((c) => c + 1)}>
      Cliqué {count} fois
    </button>
  );
}
