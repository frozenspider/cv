import { HTMLAttributes } from 'react'
import { Resume } from '../../types'
import { cn } from '@cv/lib'
import Link from 'next/link'

interface Props extends HTMLAttributes<HTMLDivElement> {
   resume: Resume
}

export const Name = ({ resume, className, ...rest }: Props) => {
   return (
      <h1 className={cn('w-full flex flex-col', className)} {...rest}>
         <p
            className="text-2xl mb-1.5 font-bold"
         >
            {resume.name}
         </p>
      </h1>
   )
}
