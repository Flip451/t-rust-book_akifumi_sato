import { FC, useState } from "react"
import { CreateLabelPayload, Label, LabelId } from "../types/label"
import { Box, IconButton, List, ListItem, ListItemButton, ListSubheader, Modal, Stack, TextField, Typography } from "@mui/material"
import { modalInnerStyle } from "../styles/modal"
import { Button } from "@mui/base"
import { Delete as DeleteIcon, Edit as EditIcon, Label as LabelIcon } from "@mui/icons-material"

type Props = {
    labels: Label[]
    filterLabelId: LabelId | null
    onSelectLabel: (label: Label | null) => void
    onSubmitCreateLabel: (newLabel: CreateLabelPayload) => void
    onDeleteLabel: (id: LabelId) => void
}

const SideNav: FC<Props> = ({
    labels,
    filterLabelId,
    onSelectLabel,
    onSubmitCreateLabel,
    onDeleteLabel,
}) => {
    const [editName, setEditName] = useState("");
    const [openLabelModal, setOpenLabelModal] = useState(false)

    const onSubmit = () => {
        setEditName("")
        onSubmitCreateLabel({ name: editName })
    }

    return (
        <>
            <List>
                <ListSubheader>Labels</ListSubheader>
                {labels.map((label) => (
                    <ListItem key={label.id} disablePadding>
                        <ListItemButton
                            onClick={() => onSelectLabel(label.id === filterLabelId ? null : label)} selected={label.id === filterLabelId}>
                            <Stack direction="row" alignItems="center" spacing={1}>
                                <LabelIcon fontSize="small" />
                                <Typography variant="caption" fontSize={18}>{label.name}</Typography>
                            </Stack>
                        </ListItemButton>
                    </ListItem>
                ))}
                <ListItem disablePadding>
                    <ListItemButton onClick={() => setOpenLabelModal(true)}>
                        <Stack direction="row" alignItems="center" spacing={1}>
                            <EditIcon fontSize="small" />
                            <Typography variant="caption" fontSize={18}>edit label</Typography>
                        </Stack>
                    </ListItemButton>
                </ListItem>
            </List>
            <Modal open={openLabelModal} onClose={() => setOpenLabelModal(false)}>
                <Box sx={modalInnerStyle}>
                    <Stack spacing={3}>
                        <Stack spacing={1}>
                            <Typography variant="subtitle1">new label</Typography>
                            <TextField
                                label="new label"
                                variant="filled"
                                fullWidth
                                onChange={(e) => setEditName(e.target.value)}
                            />
                            <Box textAlign="right">
                                <Button onClick={onSubmit}>submit</Button>
                            </Box>
                        </Stack>
                        <Stack spacing={1}>
                            {labels.map((label: Label) => (
                                <Stack
                                    key={label.id}
                                    direction="row"
                                    alignItems="center"
                                    spacing={1}
                                >
                                    <IconButton size="small" onClick={() => onDeleteLabel(label.id)}>
                                        <DeleteIcon fontSize="small" />
                                    </IconButton>
                                    <span>{label.name}</span>
                                </Stack>
                            ))}
                        </Stack>
                    </Stack>
                </Box>
            </Modal>
        </>
    )
}

export default SideNav