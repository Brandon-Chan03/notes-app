'use client';

import { ThemeProvider } from 'next-themes';
import { Sidebar } from '@/components/Global/sidebar';

export default function Home() {
  return (
    <ThemeProvider>
      <Sidebar />
      <div className='text-highlight'>TEST</div>
    </ThemeProvider>
  )
}
