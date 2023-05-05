#include <stdio.h>
#include <string.h>
#include <stdlib.h>
struct node {
	int data;
	struct node *next;
};
struct node *head = NULL;
struct node *current = NULL;

// display the list
void print_list()
{
	struct node *p = head;
	printf("\n[");

	//start from the beginning
	while (p != NULL) {
		printf(" %d ", p->data);
		p = p->next;
	}
	printf("]");
}

//insertion at the beginning
void insert_begin(int data)
{
	// create a link
	struct node *lk = (struct node *)malloc(sizeof(struct node));
	lk->data = data;

	// point it to old first node
	lk->next = head;

	//point first to new first node
	head = lk;
}
void insert_end(int data)
{
	//create a link
	struct node *lk = (struct node *)malloc(sizeof(struct node));
	lk->data = data;
	struct node *linkedlist = head;

	// point it to old first node
	while (linkedlist->next != NULL)
		linkedlist = linkedlist->next;

	//point first to new first node
	linkedlist->next = lk;
}
void insert_after(struct node *list, int data)
{
	struct node *lk = (struct node *)malloc(sizeof(struct node));
	lk->data = data;
	lk->next = list->next;
	list->next = lk;
}
void delete_begin()
{
	head = head->next;
}
void delete_end()
{
	struct node *linkedlist = head;
	while (linkedlist->next->next != NULL)
		linkedlist = linkedlist->next;
	linkedlist->next = NULL;
}
void delete_node(int key)
{
	struct node *temp = head, *prev;
	if (temp != NULL && temp->data == key) {
		head = temp->next;
		return;
	}

	// Find the key to be deleted
	while (temp != NULL && temp->data != key) {
		prev = temp;
		temp = temp->next;
	}

	// If the key is not present
	if (temp == NULL)
		return;

	// Remove the node
	prev->next = temp->next;
}
int search_list(int key)
{
	struct node *temp = head;
	while (temp != NULL) {
		if (temp->data == key) {
			return 1;
		}
		temp = temp->next;
	}
	return 0;
}
int main()
{
	int k = 0;
	insert_begin(12);
	insert_begin(22);
	insert_end(30);
	insert_end(44);
	insert_begin(50);
	insert_after(head->next->next, 33);
	printf("Linked List: ");

	print_list();

	delete_begin();
	delete_end();
	delete_node(12);
	printf("\nLinked List after deletion: ");

	print_list();

	insert_begin(4);
	insert_begin(16);
	printf("\nUpdated Linked List: ");
	print_list();
	k = search_list(16);
	if (k == 1)
		printf("\nElement is found");
	else
		printf("\nElement is not present in the list");
}
