import { HTMLAttributes } from 'react'
import { Resume } from '../../types'
import { cn } from '@cv/lib'
import Link from 'next/link'

interface Props extends HTMLAttributes<HTMLDivElement> {
   resume: Resume
}

export const Summary = ({ resume, className, ...rest }: Props) => {
   return (
      <div className={cn('w-full flex flex-col', className)} {...rest}>
         <p
            className="text-black/90 font-bold text-lg"
         >
            {resume.summary}
         </p>
      </div>
   )
}
