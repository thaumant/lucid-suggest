import {Hit} from 'lucid-suggest/en'
import React from 'react'

type ListItemProps = {
    hit: Hit,
}

const ListItem: React.FC<ListItemProps> = ({hit}) => (
    <li className="list-group-item">
        {hit.chunks.map((chunk, i) => (
            chunk.highlight
                ? <strong key={i}>{chunk.text}</strong>
                : <span key={i}>{chunk.text}</span>
        ))}
    </li>
)

export default ListItem
