import { cn } from '@cv/lib'
import { HTMLAttributes } from 'react'
import { Resume } from '../../types'
import Link from 'next/link'
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome'
import {
   faLinkedin,
   faGithub,
   faTelegram,
   faNpm,
   faTwitter,
} from '@fortawesome/free-brands-svg-icons'
import { faEnvelope, faPhone, faGlobe } from '@fortawesome/pro-solid-svg-icons'

interface Props extends HTMLAttributes<HTMLDivElement> {
   resume: Resume
}

export const Links = ({ resume, className, ...rest }: Props) => {
   if (!resume.contact.email && !resume.contact.phone) return null

   return (
      <div className={cn('flex gap-2 items-center', className)} {...rest}>
         {resume.contact.email && (
            <Link
               href={`mailto:${resume.contact.email}`}
               title={resume.contact.email}
               target="_blank"
               className="flex items-center gap-1.5 hover:underline text-sm text-black/80"
            >
               <FontAwesomeIcon icon={faEnvelope} className="h-4 w-4" />
               {resume.contact.email}
            </Link>
         )}

         {resume.contact.github && (
            <Link
               href={resume.contact.github}
               title={resume.contact.github}
               target="_blank"
               className="flex items-center gap-1.5 hover:underline text-sm text-black/80"
            >
               <FontAwesomeIcon icon={faGithub} className="h-4 w-4" />
               @{resume.contact.github.split('/').at(-1)}
            </Link>
         )}

         {resume.contact.linkedin && (
            <Link
               href={resume.contact.linkedin}
               title={resume.contact.linkedin}
               target="_blank"
               className="flex items-center gap-1.5 hover:underline text-sm text-black/80"
            >
               <FontAwesomeIcon icon={faLinkedin} className="h-4 w-4" />
               @{resume.contact.linkedin.split('/').at(-1)}
            </Link>
         )}
      </div>
   )
}
